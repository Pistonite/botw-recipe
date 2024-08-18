# find_recipes.py Version 1.2
# 
# Copyright 2022 brkirch
# 
# Redistribution and use in source and binary forms, with or without modification, are permitted provided that the following conditions are met:
# 
# 1. Redistributions of source code must retain the above copyright notice, this list of conditions and the following disclaimer.
# 
# 2. Redistributions in binary form must reproduce the above copyright notice, this list of conditions and the following disclaimer in the documentation and/or other materials provided with the distribution.
# 
# THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT HOLDER OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
# 
# Changelog:
# Version 1.2 (8/9/22)
# - An updated recipeData.json is required for this version
# - Added key items
# Version 1.1 (8/1/22)
# - An updated recipeData.json is required for this version
# - Updated to detect when crit cook will always apply
# - Added perentage indicator when iterating through combinations for manually specified recipes
# Version 1.0.2 (8/1/22)
# - Fix Dragon Claws not being included with Dragon Parts
# Version 1.0.1 (8/1/22)
# - Minor addition to manual recipe/ingredient entry parsing
# Version 1.0.0 (8/1/22)
# - More changes and enhancements to manual recipe entry parsing
# - Add flags for specifying which materials can or can't be used as ingredients
# Version 1.0.0a (7/30/22)
# - Fix broken algorithm for iterating through combinations for manually specified recipes
# - Further increase flexibility of manual recipe entry; now includes fuzzy matching for material names and the ability to specify entire material food/material categories
# Version 0.9.1 (7/28/22)
# - Minor improvements to manual recipe entry parsing
# Version 0.9 (7/28/22)
# - Significantly increased the flexibility of the manual recipe entry syntax. Using delimiters / | ; : and also " or " between ingredients specifies that any one of the ingredients can be used, and all possible recipes that can be created from the combinations will be tested.
# Version 0.8 (7/27/22)
# - Added a flag for manual recipe entry (any conditions specified will still be accounted for)
# - Fixed more recipe matching bugs
# Version 0.7 (7/27/22)
# - Fixed a recipe matching bug that affects a few recipes

import numpy as np
import json
from pathlib import Path
from enum import IntFlag
import argparse
import sys
import math
import re
import difflib

class weaponModifiers(IntFlag):
    GuardUp = 0b100000000
    SurfUp = 0b010000000
    QuickShot = 0b001000000
    Zoom = 0b000100000
    MultiShot = 0b000010000
    LongThrow = 0b000001000
    CriticalHit = 0b000000100
    DurabilityUp = 0b000000010
    AttackUp = 0b000000001

def checkVersion(recipeData):
    if isinstance(recipeData[-1], int):
        return recipeData[-1]
    else:
        return 0

def getMaterialNames(materialType):
    if materialType == 'horn':
        return ["Shard of Dinraal's Horn", "Shard of Farosh's Horn", "Shard of Naydra's Horn"]
    elif materialType == 'fang':
        return ["Shard of Dinraal's Fang", "Shard of Farosh's Fang", "Shard of Naydra's Fang"]
    elif materialType == 'claw':
        return ["Dinraal's Claw", "Farosh's Claw", "Naydra's Claw"]
    elif materialType == 'scale':
        return ["Dinraal's Scale", "Farosh's Scale", "Naydra's Scale"]
    elif materialType == 'meat':
        return [material['Name'] for material in materialData if 'CookMeat' in material['Tags']]
    elif materialType == 'fruit':
        return [material['Name'] for material in materialData if 'CookFruit' in material['Tags']]
    elif materialType == 'vegetable':
        return ["Endura Carrot", "Big Hearty Radish", "Swift Carrot", "Hearty Radish", "Fortified Pumpkin"]
    elif materialType == 'herb':
        return [material['Name'] for material in materialData if 'CookPlant' in material['Tags'] and not material['Name'] in getMaterialNames('vegetable')]
    elif materialType == 'insect':
        return [material['Name'] for material in materialData if 'CookInsect' in material['Tags'] and not material['Name'] in getMaterialNames('animal')]
    elif materialType == 'lizard':
        return ["Hearty Lizard", "Fireproof Lizard", "Hightail Lizard"]
    elif materialType == 'frog':
        return ["Tireless Frog", "Hot-Footed Frog"]
    elif materialType == 'animal':
        return getMaterialNames('lizard') + getMaterialNames('frog')
    elif materialType == 'critter':
        return [material['Name'] for material in materialData if 'CookInsect' in material['Tags']]
    elif materialType == 'monster part':
        return [material['Name'] for material in materialData if 'CookEnemy' in material['Tags']]
    elif materialType == 'roasted meat':
        return [material['Name'] for material in materialData if 'RoastMeat' in material['Tags'] and material['EffectType'] != "ResistHot"]
    elif materialType == 'frozen meat':
        return [material['Name'] for material in materialData if 'RoastMeat' in material['Tags'] and material['EffectType'] == "ResistHot"]
    elif materialType == 'roasted vegetable':
        return [material['Name'] for material in materialData if 'RoastVegetable' in material['Tags']]
    elif materialType == 'roasted fruit':
        return [material['Name'] for material in materialData if 'RoastFruit' in material['Tags']]
    elif materialType == 'roasted food':
        return [material['Name'] for material in materialData if 'RoastItem' in material['Tags']]
    elif materialType == 'additive':
        return [material['Name'] for material in materialData if 'CookSpice' in material['Tags']]
    elif materialType == 'food':
        return getMaterialNames('meat') + getMaterialNames('fruit') + getMaterialNames('vegetable') + getMaterialNames('herb')
    elif materialType == 'key item':
        return [material['Name'] for material in materialData if 'Important' in material['Tags']]
    elif materialType == 'material':
        return [material['Name'] for material in materialData]

def parseIngredient(ingredient):
    ingredient = ingredient.lower()
    if (matchedString := " without ") in ingredient or (matchedString := " excluding ") in ingredient or (matchedString := " not including ") in ingredient or (matchedString := " except for ") in ingredient or (matchedString := " minus ") in ingredient:
        ingredientsList = parseIngredient(ingredient[:ingredient.index(matchedString)])
        exclusionsList = []
        for exclusion in re.split(';| and ', ingredient[ingredient.index(matchedString) + len(matchedString):]):
            exclusionsList += parseIngredient(exclusion)
        ingredientsListCopy = ingredientsList.copy()
        ingredientsList = []
        for ingredientIndex, ingredient in enumerate(ingredientsListCopy):
            if not ingredient in exclusionsList: ingredientsList.append(ingredient)
        return ingredientsList
    ingredientsList = []
    if ingredient.startswith("any ") or ingredient.startswith("a ") or ingredient.startswith("an ") or ingredient.startswith("all "): ingredient = ingredient.split(' ', 1)[1]
    if (ingredient.endswith(" item") and not ingredient.endswith("key item")) or (ingredient.endswith(" items") and not ingredient.endswith("key items")) or ingredient.endswith(" material") or ingredient.endswith(" materials") or ingredient.endswith(" mat") or ingredient.endswith(" mats") or ingredient.endswith(" ingredient") or ingredient.endswith(" ingredients"): ingredient = ingredient.rsplit(' ', 1)[0]
    if ingredient == "dragon horn" or ingredient == "dragon horns" or ingredient == "horn" or ingredient == "horns":
        ingredientsList += getMaterialNames('horn')
    elif ingredient == "dragon fang" or ingredient == "dragon fangs" or ingredient == "fang" or ingredient == "fangs":
        ingredientsList += getMaterialNames('fang')
    elif ingredient == "dragon claw" or ingredient == "dragon claws" or ingredient == "claw" or ingredient == "claws":
        ingredientsList += getMaterialNames('claw')
    elif ingredient == "dragon scale" or ingredient == "dragon scales" or ingredient == "scale" or ingredient == "scales":
        ingredientsList += getMaterialNames('scale')
    elif ingredient == "dragon part" or ingredient == "dragon parts":
        ingredientsList += getMaterialNames('horn') + getMaterialNames('fang') + getMaterialNames('claw') + getMaterialNames('scale')
    elif ingredient == "meat" or ingredient == "meats":
        ingredientsList += getMaterialNames('meat')
    elif ingredient == "fruit" or ingredient == "fruits":
        ingredientsList += getMaterialNames('fruit')
    elif ingredient == "vegetable" or ingredient == "vegetables" or ingredient == "veggie" or ingredient == "veggies":
        ingredientsList += getMaterialNames('vegetable')
    elif ingredient == "herb" or ingredient == "herbs" or ingredient == "plant" or ingredient == "plants":
        ingredientsList += getMaterialNames('herb')
    elif ingredient == "insect" or ingredient == "insects":
        ingredientsList += getMaterialNames('insect')
    elif ingredient == "lizard" or ingredient == "lizards":
        ingredientsList += getMaterialNames('lizard')
    elif ingredient == "frog" or ingredient == "frogs":
        ingredientsList += getMaterialNames('frog')
    elif ingredient == "animal" or ingredient == "animals":
        ingredientsList += getMaterialNames('animal')
    elif ingredient == "critter" or ingredient == "critters":
        ingredientsList += getMaterialNames('critter')
    elif ingredient == "monster part" or ingredient == "monster parts":
        ingredientsList += getMaterialNames('monster part')
    elif ingredient == "roasted meat" or ingredient == "roasted meats" or ingredient == "seared meat" or ingredient == "seared meats" or ingredient == "cooked meat" or ingredient == "cooked meats":
        ingredientsList += getMaterialNames('roasted meat')
    elif ingredient == "frozen meat" or ingredient == "frozen meats":
        ingredientsList += getMaterialNames('frozen meat')
    elif ingredient == "roasted vegetable" or ingredient == "roasted vegetables" or ingredient == "roasted veggie" or ingredient == "roasted veggies" or ingredient == "cooked vegetable" or ingredient == "cooked vegetables" or ingredient == "cooked veggie" or ingredient == "cooked veggies":
        ingredientsList += getMaterialNames('roasted vegetable')
    elif ingredient == "roasted fruit" or ingredient == "roasted fruits" or ingredient == "cooked fruit" or ingredient == "cooked fruits":
        ingredientsList += getMaterialNames('roasted fruit')
    elif ingredient == "roasted food" or ingredient == "roasted foods" or ingredient == "cooked food" or ingredient == "cooked foods":
        ingredientsList += getMaterialNames('roasted food')
    elif ingredient == "additive" or ingredient == "additives":
        ingredientsList += getMaterialNames('additive')
    elif ingredient == "food" or ingredient == "foods":
        ingredientsList += getMaterialNames('food')
    elif ingredient == "material" or ingredient == "materials" or ingredient == "mat" or ingredient == "mats":
        ingredientsList += getMaterialNames('material')
    elif ingredient == "key item" or ingredient == "key items":
        ingredientsList += getMaterialNames('key item')
    elif ingredient.lower() == "slate":
        ingredientsList.append("Sheikah Slate")
    elif ingredient.lower() == "glider":
        ingredientsList.append("Paraglider")
    elif ingredient.lower() == "octorok balloon":
        ingredientsList.append("Octo Balloon")
    elif ingredient.lower() == "fairies":
        ingredientsList.append("Fairy")
    else:
        materialNames = [material['Name'].lower() for material in materialData]
        if not ingredient in materialNames:
            if ingredient.endswith('s') and ingredient[:-1] in materialNames: return [materialData[materialNames.index(ingredient[:-1])]['Name']]
            if ingredient.endswith('es') and ingredient[:-2] in materialNames: return [materialData[materialNames.index(ingredient[:-2])]['Name']]
            ingredientGuess = difflib.get_close_matches(ingredient, materialNames, 1)
            if len(ingredientGuess) > 0:
                ingredient = ingredientGuess[0]
            else:
                ingredientGuess = difflib.get_close_matches(ingredient, ["dragon horn", "dragon horns", "horn", "horns", "dragon fang", "dragon fangs", "fang", "fangs", "dragon claw", "dragon claws", "claw", "claws", "dragon scale", "dragon scales", "scale", "scales", "dragon part", "dragon parts", "meat", "meats", "fruit", "fruits", "vegetable", "vegetables", "veggie", "veggies", "herb", "herbs", "plant", "plants", "insect", "insects", "lizard", "lizards", "frog", "frogs", "animal", "animals", "critter", "critters", "monster part", "monster parts", "roasted meat", "roasted meats", "seared meat", "seared meats", "cooked meat", "cooked meats", "frozen meat", "frozen meats", "roasted vegetable", "roasted vegetables", "cooked vegetable", "cooked vegetables", "roasted veggie", "roasted veggies", "cooked veggie", "cooked veggies", "roasted fruit", "roasted fruits", "cooked fruit", "cooked fruits", "roasted food", "roasted foods", "cooked food", "cooked foods", "food", "foods", "material", "materials", "mat", "mats", "key item", "key items"], 1)
                if len(ingredientGuess) > 0:
                    return parseIngredient(ingredientGuess[0])
                else:
                    raise Exception("\"" + ingredient + "\" is not a recognized material name")
        return [materialData[materialNames.index(ingredient)]['Name']]
    return ingredientsList

def parseRecipe(recipe):
    materialsList = []
    recipeSlots = [ingredient.strip() for ingredient in recipe.split(',')]
    if len(recipeSlots) > 5: raise Exception("Recipes can have at most 5 ingredients")
    recipeSlotsLen = len(recipeSlots)
    recipeDifferGroups = [None for _ in range(recipeSlotsLen)]
    recipeSameGroups = [None for _ in range(recipeSlotsLen)]
    recipeDifferGroupNum, recipeSameGroupNum = 0, 0
    for recipeSlotIndex in range(recipeSlotsLen):
        def doMaterialAppend(materialNames, differGroup):
            nonlocal recipeSlots, recipeSlotIndex, recipeDifferGroups, recipeDifferGroupNum, recipeSameGroups, recipeSameGroupNum
            for materialNameIndex, materialName in enumerate(materialNames):
                if materialNameIndex == 0:
                    recipeSlots[recipeSlotIndex] = materialName.lower()
                else:
                    recipeSlots.append(materialName.lower())
                    if differGroup:
                        recipeDifferGroups.append(recipeDifferGroupNum)
                        recipeSameGroups.append(None)
                    else:
                        recipeDifferGroups.append(None)
                        recipeSameGroups.append(recipeSameGroupNum)
            if len(materialNames) > 1 and differGroup:
                recipeDifferGroups[recipeSlotIndex] = recipeDifferGroupNum
                recipeDifferGroupNum += 1
            elif len(materialNames) > 1:
                recipeSameGroups[recipeSlotIndex] = recipeSameGroupNum
                recipeSameGroupNum += 1
        recipeSlot = recipeSlots[recipeSlotIndex]
        def checkForPrefix(numAppend):
            nonlocal recipeSlot, recipeSlotIndex, recipeSlots
            if recipeSlots[recipeSlotIndex].strip().lower().startswith("different ") or recipeSlots[recipeSlotIndex].strip().lower().startswith("distinct ") or recipeSlots[recipeSlotIndex].strip().lower().startswith("unique ") or recipeSlots[recipeSlotIndex].strip().lower().startswith("differing "):
                recipeSlots[recipeSlotIndex] = recipeSlots[recipeSlotIndex].strip().split(' ', 1)[1]
                doMaterialAppend([recipeSlots[recipeSlotIndex] for _ in range(numAppend)], True)
            elif recipeSlots[recipeSlotIndex].strip().lower().startswith("same ") or recipeSlots[recipeSlotIndex].strip().lower().startswith("of the same ") or recipeSlots[recipeSlotIndex].strip().lower().startswith("identical ") or recipeSlots[recipeSlotIndex].strip().lower().startswith("like ") or recipeSlots[recipeSlotIndex].strip().lower().startswith("alike ") or recipeSlots[recipeSlotIndex].strip().lower().startswith("matching "):
                if recipeSlots[recipeSlotIndex].strip().lower().startswith("of the same "):
                    recipeSlots[recipeSlotIndex] = recipeSlots[recipeSlotIndex].strip()[12:]
                else:
                    recipeSlots[recipeSlotIndex] = recipeSlots[recipeSlotIndex].strip().split(' ', 1)[1]
                doMaterialAppend([recipeSlots[recipeSlotIndex] for _ in range(numAppend)], False)
            else:
                return True
            return False
        if len(recipeSlot) > 1:
            if (ingredientCount := recipeSlot[0]).isdigit() and ingredientCount != '0':
                if recipeSlot[1].lower() == 'x':
                    recipeSlots[recipeSlotIndex] = recipeSlot[2:]
                else:
                    recipeSlots[recipeSlotIndex] = recipeSlot[1:]
                if checkForPrefix(int(ingredientCount)):
                    for _ in range(int(ingredientCount) - 1):
                        recipeSlots.append(recipeSlots[recipeSlotIndex])
                        recipeDifferGroups.append(None)
                        recipeSameGroups.append(None)
            elif (ingredientCount := recipeSlot[-1]).isdigit() and ingredientCount != '0' and recipeSlot[-2].lower() == 'x':
                recipeSlots[recipeSlotIndex] = recipeSlot[:-2]
                for _ in range(int(ingredientCount) - 1):
                    recipeSlots.append(recipeSlots[recipeSlotIndex])
                    recipeDifferGroups.append(None)
                    recipeSameGroups.append(None)
            elif recipeSlot.lower().startswith("one "):
                recipeSlots[recipeSlotIndex] = recipeSlot[4:]
            elif recipeSlot.lower().startswith("two "):
                recipeSlots[recipeSlotIndex] = recipeSlot[4:]
                if checkForPrefix(2):
                    for _ in range(1):
                        recipeSlots.append(recipeSlots[recipeSlotIndex])
                        recipeDifferGroups.append(None)
                        recipeSameGroups.append(None)
            elif recipeSlot.lower().startswith("three "):
                recipeSlots[recipeSlotIndex] = recipeSlot[6:]
                if checkForPrefix(3):
                    for _ in range(2):
                        recipeSlots.append(recipeSlots[recipeSlotIndex])
                        recipeDifferGroups.append(None)
                        recipeSameGroups.append(None)
            elif recipeSlot.lower().startswith("four "):
                recipeSlots[recipeSlotIndex] = recipeSlot[5:]
                if checkForPrefix(4):
                    for _ in range(3):
                        recipeSlots.append(recipeSlots[recipeSlotIndex])
                        recipeDifferGroups.append(None)
                        recipeSameGroups.append(None)
            elif recipeSlot.lower().startswith("five "):
                recipeSlots[recipeSlotIndex] = recipeSlot[5:]
                if checkForPrefix(5):
                    for _ in range(4):
                        recipeSlots.append(recipeSlots[recipeSlotIndex])
                        recipeDifferGroups.append(None)
                        recipeSameGroups.append(None)
    if len(recipeSlots) > 5: raise Exception("Recipes can have at most 5 ingredients")
    for recipeSlot in recipeSlots:
        materialsListSlot = []
        recipeSlot = [ingredient.strip() for ingredient in re.split('/|\\||:| or ', recipeSlot.lower())]
        recipeSlotLen = len(recipeSlot)
        for recipeSlotIndex in range(recipeSlotLen):
            ingredientsList = parseIngredient(recipeSlot[recipeSlotIndex])
            recipeSlot[recipeSlotIndex] = ingredientsList[0].lower()
            if len(ingredientsList) > 1:
                for ingredient in ingredientsList[1:]:
                    recipeSlot.append(ingredient.lower())
        for ingredient in recipeSlot:
            for materialIndex in range(len(materialData)):
                if materialData[materialIndex]['Name'].lower() == ingredient:
                    break
            else:
                raise Exception("\"" + ingredient + "\" is not a recognized material name")
            materialsListSlot.append(materialIndex)
        materialsList.append(materialsListSlot)
    return (materialsList, recipeDifferGroups, recipeSameGroups)

def getPrice(sellTotal, buyTotal, ingredientCount):
    if ingredientCount > 5:
        ingredientCount = 5
    sellTotal = int(np.float32(sellTotal) * NMMR[ingredientCount - 1])
    if sellTotal % 10 != 0:
        sellTotal += 10
        sellTotal -= (sellTotal % 10)
    if buyTotal < sellTotal:
        sellTotal = buyTotal
    if sellTotal < 3:
        sellTotal = 2
    return sellTotal

def addRecipePrefix(recipeName, effectType):
    if effectType == "ResistHot":
        recipeName = "Chilly " + recipeName
    elif effectType == "ResistCold":
        recipeName = "Spicy " + recipeName
    elif effectType == "ResistElectric":
        recipeName = "Electro " + recipeName
    elif effectType == "Quietness":
        recipeName = "Sneaky " + recipeName
    elif effectType == "GutsRecover":
        recipeName = "Energizing " + recipeName
    elif effectType == "ExGutsMaxUp":
        recipeName = "Enduring " + recipeName
    elif effectType == "MovingSpeed":
        recipeName = "Hasty " + recipeName
    elif effectType == "AttackUp":
        recipeName = "Mighty " + recipeName
    elif effectType == "DefenseUp":
        recipeName = "Tough " + recipeName
    elif effectType == "Fireproof":
        recipeName = "Fireproof " + recipeName
    elif effectType == "LifeMaxUp":
        recipeName = "Hearty " + recipeName
    return recipeName

def verifyRecipe(materialsList: list):
    effectType, cookValue, critChance, numUnique, recipe, recipeList, recipeTags, ingredientIndexes = False, 0, 0, 0, [], [], [], []
    materialsList = materialsList.copy()
    for materialNameIndex in range(len(materialsList)):
        if len(materialsList[materialNameIndex]) > 1:
            if materialNameIndex > 0 and materialsList[materialNameIndex-1][0] in materialsList[materialNameIndex]: 
                materialsList[materialNameIndex] = \
                [materialsList[materialNameIndex][materialTruncateIndex] for materialTruncateIndex in range(materialsList[materialNameIndex].index(materialsList[materialNameIndex-1][0]), len(materialsList[materialNameIndex]))]
            materialNameList = materialsList[materialNameIndex]
            for materialName in materialNameList:
                materialsList[materialNameIndex] = [materialName]
                recipeList += verifyRecipe(materialsList)
            return recipeList
    for materialNameIndex in range(len(materialsList)):
        material = materialData[materialsList[materialNameIndex][0]]
        ingredientIndexes.append(materialsList[materialNameIndex][0])
        recipe.append(material['Name'])
        if material['EffectType'] != "None":
            if effectType == False and material['EffectType'] != "None":
                effectType = material['EffectType']
            elif effectType != "None" and effectType != False and material['EffectType'] != effectType:
                effectType = "None"
        cookValue += material['HitPointRecover'] * 2
        if materialsList.index(materialsList[materialNameIndex]) == materialNameIndex:
            cookValue += material['BoostHitPointRecover']
            critChance += material['BoostSuccessRate']
            numUnique += 1
        recipeTags.append(material['Tags'])
    if cookValue > 120: cookValue = 120
    if critChance > 100: critChance = 100
    if numUnique == 1:
        for checkRecipe in cookData['SingleRecipes']:
            hasMatched = True
            recipeIndexes = set(range(len(recipe)))
            if 'Actors' in checkRecipe and checkRecipe['Actors'] != []:
                hasMatched = False
                for ingredient in checkRecipe['Actors']:
                    for recipeIndex in recipeIndexes:
                        if recipe[recipeIndex] == ingredient:
                            ingredientName = recipe[recipeIndex]
                            recipeIndexes = [recipeIndex for recipeIndex in recipeIndexes if not recipe[recipeIndex] == ingredientName]
                            hasMatched = True
                            break
                    if hasMatched: break
                if not hasMatched: continue
            if 'Tags' in checkRecipe and checkRecipe['Tags'] != []:
                hasMatched = False
                for tag in checkRecipe['Tags']:
                    if tag == []:
                        continue
                    else:
                        tag = tag[0]
                    for recipeIndex in recipeIndexes:
                        if tag in recipeTags[recipeIndex]:
                            ingredientName = recipe[recipeIndex]
                            recipeIndexes = [recipeIndex for recipeIndex in recipeIndexes if not recipe[recipeIndex] == ingredientName]
                            hasMatched = True
                            break
                    if hasMatched: break
                if not hasMatched: continue
            if checkRecipe['Recipe'] == "Fairy Tonic": return []
            recipeName = addRecipePrefix(checkRecipe['Recipe'], effectType)
            if recipeName == "Elixir": return []
            if 'HB' in checkRecipe: cookValue += int(checkRecipe['HB'])
            if critChance == 100 and (effectType == "None" or effectType == False):
                cookValue += 12
                if cookValue > 120: cookValue = 120
            return [{'recipeName': recipeName, 'recipe': recipe, 'recipeIndexes': ingredientIndexes, 'cookValue': cookValue, 'critChance': critChance, 'effectType': effectType}]

    for checkRecipe in cookData['Recipes']:
        hasMatched = True
        recipeIndexes = set(range(len(recipe)))
        if 'Actors' in checkRecipe:
            for ingredientGroup in checkRecipe['Actors']:
                if ingredientGroup == []: continue
                hasMatched = False
                for ingredient in ingredientGroup:
                    for recipeIndex in recipeIndexes:
                        if recipe[recipeIndex] == ingredient:
                            ingredientName = recipe[recipeIndex]
                            recipeIndexes = [recipeIndex for recipeIndex in recipeIndexes if not recipe[recipeIndex] == ingredientName]
                            hasMatched = True
                            break
                    if hasMatched: break
                if not hasMatched: break
            if not hasMatched: continue
        if 'Tags' in checkRecipe:
            for tagsGroup in checkRecipe['Tags']:
                if tagsGroup == []: continue
                hasMatched = False
                for tag in tagsGroup:
                    if tag == []:
                        continue
                    else:
                        tag = tag[0]
                    for recipeIndex in recipeIndexes:
                        if tag in recipeTags[recipeIndex]:
                            ingredientName = recipe[recipeIndex]
                            recipeIndexes = [recipeIndex for recipeIndex in recipeIndexes if not recipe[recipeIndex] == ingredientName]
                            hasMatched = True
                            break
                    if hasMatched: break
                if not hasMatched: break
            if not hasMatched: continue
        if checkRecipe['Recipe'] == "Fairy Tonic" or checkRecipe['Recipe'] == "Dubious Food" or checkRecipe['Recipe'] == "Dubious Food" or checkRecipe['Recipe'] == "Rock-Hard Food": return []
        recipeName = addRecipePrefix(checkRecipe['Recipe'], effectType)
        if recipeName == "Elixir": return []
        if 'HB' in checkRecipe: cookValue += int(checkRecipe['HB'])
        if critChance == 100 and (effectType == "None" or effectType == False):
            cookValue += 12
            if cookValue > 120: cookValue = 120
        return [{'recipeName': recipeName, 'recipe': recipe, 'recipeIndexes': ingredientIndexes, 'cookValue': cookValue, 'critChance': critChance, 'effectType': effectType}]
    return []
    

def iterateRecipes(optimizedMaterialList, inclusionMask, exclusionMask, numRoastedAllowed_, minVal, zeroVal, numOfRecipes, firstIterationEndIndex, buyTotal_ = 0, sellTotal_ = 0, materialsList_ = [], numIngredientsLeft_ = 5, startIndex = 0, results = []):
    for materialIndex in range(startIndex, firstIterationEndIndex):
        if numOfRecipes is not None and len(results) >= numOfRecipes: break
        if numIngredientsLeft_ == 5:
            percentLeft = int(((((materialIndex - startIndex)*len(optimizedMaterialList)**4)/math.factorial(5)) / (((firstIterationEndIndex - startIndex)*len(optimizedMaterialList)**4)/math.factorial(5)))*100)
            print(f'{str(percentLeft) + "%": >4}', file=sys.stderr, end='\r')
        material = optimizedMaterialList[materialIndex]
        buyTotal = buyTotal_
        sellTotal = sellTotal_
        numRoastedAllowed = numRoastedAllowed_
        materialsList = materialsList_
        numIngredientsLeft = numIngredientsLeft_
        if material[1] == "RoastItem" or material[1] == "Important":
            if numRoastedAllowed < 1:
                continue
            else:
                numRoastedAllowed -= 1
        buyTotal += material[2]
        sellTotal += material[3]
        materialsList = materialsList.copy()
        materialsList.append(material[0])
        numIngredientsLeft -= 1
        price = getPrice(sellTotal, buyTotal, 5 - numIngredientsLeft)
        if inclusionMask & price == inclusionMask and exclusionMask & price == 0:
            resultRecipes = verifyRecipe(materialsList)
            for resultRecipe in resultRecipes:
                valWithCrit = resultRecipe['cookValue']
                if resultRecipe['critChance'] < 100 or (resultRecipe['effectType'] != "None" and resultRecipe['effectType'] != False): valWithCrit += 12
                if valWithCrit > 120: valWithCrit = 120
                if resultRecipe['effectType'] != "LifeMaxUp" and ((not zeroVal and valWithCrit >= minVal) or (zeroVal and resultRecipe['cookValue'] == 0)):
                    results.append((valWithCrit, resultRecipe['cookValue'], price, resultRecipe['recipe'],resultRecipe['recipeName']))
                    if numOfRecipes is not None and len(results) >= numOfRecipes: return results
        if numIngredientsLeft > 0: iterateRecipes(optimizedMaterialList, inclusionMask, exclusionMask, numRoastedAllowed, minVal, zeroVal, numOfRecipes, len(optimizedMaterialList), buyTotal, sellTotal, materialsList, numIngredientsLeft, materialIndex, results)
    return results

def findRecipes(inclusionMask, exclusionMask = 0, numRoastedAllowed = 4, minVal = 0, zeroVal = False, allowElixirs = False, numOfRecipes = None, ingredientsToUse = None, ingredientsNotToUse = None):
    if ingredientsToUse is not None:
        ingredientsToUse = ingredientsToUse.split(',')
        useIngredientList = []
        for ingredient in ingredientsToUse:
            useIngredientList += parseIngredient(ingredient.strip())
    else:
        useIngredientList = None
    if ingredientsNotToUse is not None:
        ingredientsNotToUse = ingredientsNotToUse.split(',')
        notUseIngredientList = []
        for ingredient in ingredientsNotToUse:
            notUseIngredientList += parseIngredient(ingredient.strip())
    else:
        notUseIngredientList = None
    optimizedMaterialList = []
    for materialIndex in range(len(materialData)):
        material = materialData[materialIndex]
        if useIngredientList is not None and not material['Name'] in useIngredientList: continue
        if notUseIngredientList is not None and material['Name'] in notUseIngredientList: continue
        if "RoastItem" in material['Tags']:
            materialType = "RoastItem"
        elif "Important" in material['Tags']:
            materialType = "Important"
        elif "Fairy" in material['Tags']:
            materialType = "Fairy"
        elif "CookSpice" in material['Tags']:
            materialType = "CookSpice"
        elif "CookPlant" in material['Tags']:
            materialType = "CookPlant"
        elif "CookMushroom" in material['Tags']:
            materialType = "CookMushroom"
        elif "CookMeat" in material['Tags']:
            materialType = "CookMeat"
        elif "CookFish" in material['Tags']:
            materialType = "CookFish"
        elif "CookFruit" in material['Tags']:
            materialType = "CookFruit"
        elif allowElixirs and "CookEnemy" in material['Tags']:
            materialType = "CookEnemy"
        elif allowElixirs and "CookInsect" in material['Tags']:
            materialType = "CookInsect"
        else:
            continue
        if "CookLowPrice" in material['Tags']:
            material = material.copy()
            material['BuyingPrice'] = 1
            material['SellingPrice'] = 1
        alreadyAdded = False
        for optimizedMaterial in optimizedMaterialList:
            if optimizedMaterial[1] == materialType and optimizedMaterial[2] == material['BuyingPrice'] and optimizedMaterial[3] == material['SellingPrice'] and optimizedMaterial[4] == material['EffectType']:
                optimizedMaterial[0].append(materialIndex)
                alreadyAdded = True
        if not alreadyAdded: optimizedMaterialList.append([[materialIndex], materialType, material['BuyingPrice'], material['SellingPrice'], material['EffectType']])
    if inclusionMask & 1 != 0:
        firstIterationEndIndex = 0
        for optimizedmaterialIndex in range(len(optimizedMaterialList)):
            if optimizedMaterialList[optimizedmaterialIndex][2] & 1 != 0:
                optimizedMaterialList.insert(0, optimizedMaterialList.pop(optimizedmaterialIndex))
                firstIterationEndIndex += 1
    else:
        if allowElixirs:
            for optimizedmaterialIndex in range(len(optimizedMaterialList)):
                if optimizedMaterialList[optimizedmaterialIndex][1] == "CookEnemy" or optimizedMaterialList[optimizedmaterialIndex][1] == "CookInsect":
                    optimizedMaterialList.insert(0, optimizedMaterialList.pop(optimizedmaterialIndex))
        firstIterationEndIndex = len(optimizedMaterialList)
    return iterateRecipes(optimizedMaterialList, inclusionMask, exclusionMask, numRoastedAllowed, minVal, zeroVal, numOfRecipes, firstIterationEndIndex)

parser = argparse.ArgumentParser(prefix_chars='-+')

parser.add_argument('-F', dest='recipeDataPath', default='recipeData.json', help='path to recipeData.json')
parser.add_argument('-O', dest='csvOutPath', help='path to save results CSV file (defaults to "BOTW WMC Recipes [Modifiers].csv")')
parser.add_argument('-C', '--cookedAndKeyItems', type=int, dest='cookedFoods', default=4, help='maximum number of cooked foods and key items (default is 4)', choices=range(0, 5))
parser.add_argument('-E', '--allowElixirs', dest='allowElixirs', action='store_true', help='search for elixirs (default is to exclude elixirs)')
parser.add_argument('-L', '--limitResults', dest='numOfRecipes', type=int, default=10000, help='maximum number of recipes to return (default is 10000, set to 0 for no limit)')
parser.add_argument('-R', '--recipes', dest='recipes', nargs='+', help='recipe(s) to process, using format ingredient1[,ingredient2[,ingredient3[,ingredient4[,ingredient5]]]] (recipes with Hearty results, as well as Fairy Tonics and cooking failures will be ignored)')

parser.add_argument('+I', dest='ingredientsToUse', help='search only listed ingredients (comma delimited)')
parser.add_argument('-I', dest='ingredientsNotToUse', help='search without the listed ingredients (comma delimited)')

valueLimiters = parser.add_mutually_exclusive_group()
valueLimiters.add_argument('-M', '--min', type=int, dest='minVal', default=0, help='minimum value for recipe (with crit)')
valueLimiters.add_argument('-Z', '--zero', dest='zeroVal', action='store_true', help='search for only zero value recipes')

parser.add_argument('-P', '--price', type=int, help='price for recipe')
parser.add_argument('-X', '--exclude', dest='excludeUnspecified', action='store_true', help='require recipe price to exclude any modifier flags not explicitly specified')

guardUpGroup = parser.add_mutually_exclusive_group()
guardUpGroup.add_argument('+g', '++GuardUp', dest='enableGuardUp', help='require recipe price to have Guard Up modifier flag', action='store_true')
guardUpGroup.add_argument('-g', '--GuardUp', dest='disableGuardUp', help='require recipe price to not have Guard Up modifier flag', action='store_true')

surfUpGroup = parser.add_mutually_exclusive_group()
surfUpGroup.add_argument('+s', '++SurfUp', dest='enableSurfUp', help='require recipe price to have Surf Up modifier flag', action='store_true')
surfUpGroup.add_argument('-s', '--SurfUp', dest='disableSurfUp', help='require recipe price to not have Surf Up modifier flag', action='store_true')

quickShotGroup = parser.add_mutually_exclusive_group()
quickShotGroup.add_argument('+q', '++QuickShot', dest='enableQuickShot', help='require recipe price to have Quick Shot modifier flag', action='store_true')
quickShotGroup.add_argument('-q', '--QuickShot', dest='disableQuickShot', help='require recipe price to not have Quick Shot modifier flag', action='store_true')

zoomGroup = parser.add_mutually_exclusive_group()
zoomGroup.add_argument('+z', '++Zoom', dest='enableZoom', help='require recipe price to have Zoom modifier flag', action='store_true')
zoomGroup.add_argument('-z', '--Zoom', dest='disableZoom', help='require recipe price to not have Zoom modifier flag', action='store_true')

multiShotGroup = parser.add_mutually_exclusive_group()
multiShotGroup.add_argument('+m', '++MultiShot', dest='enableMultiShot', help='require recipe price to have Multi-Shot modifier flag', action='store_true')
multiShotGroup.add_argument('-m', '--MultiShot', dest='disableMultiShot', help='require recipe price to not have Multi-Shot modifier flag', action='store_true')

longThrowGroup = parser.add_mutually_exclusive_group()
longThrowGroup.add_argument('+l', '++LongThrow', dest='enableLongThrow', help='require recipe price to have Long Throw modifier flag', action='store_true')
longThrowGroup.add_argument('-l', '--LongThrow', dest='disableLongThrow', help='require recipe price to not have Long Throw modifier flag', action='store_true')

criticalHitGroup = parser.add_mutually_exclusive_group()
criticalHitGroup.add_argument('+c', '++CriticalHit', dest='enableCriticalHit', help='require recipe price to have Critical Hit modifier flag', action='store_true')
criticalHitGroup.add_argument('-c', '--CriticalHit', dest='disableCriticalHit', help='require recipe price to not have Critical Hit modifier flag', action='store_true')

durabilityUpGroup = parser.add_mutually_exclusive_group()
durabilityUpGroup.add_argument('+d', '++DurabilityUp', dest='enableDurabilityUp', help='require recipe price to have Durability Up modifier flag', action='store_true')
durabilityUpGroup.add_argument('-d', '--DurabilityUp', dest='disableDurabilityUp', help='require recipe price to not have Durability Up modifier flag', action='store_true')

attackUpGroup = parser.add_mutually_exclusive_group()
attackUpGroup.add_argument('+a', '++AttackUp', dest='enableAttackUp', help='require recipe price to have Attack Up modifier flag', action='store_true')
attackUpGroup.add_argument('-a', '--AttackUp', dest='disableAttackUp', help='require recipe price to not have Attack modifier flag', action='store_true')

args = parser.parse_args()

if args.price is not None:
    inclusionMask = args.price & 0b111111111
    exclusionMask = 0b111111111 ^ (args.price & 0b111111111)
else:
    inclusionMask = 0
    exclusionMask = 0

if args.enableGuardUp: inclusionMask |= weaponModifiers.GuardUp
if args.disableGuardUp: exclusionMask |= weaponModifiers.GuardUp

if args.enableSurfUp: inclusionMask |= weaponModifiers.SurfUp
if args.disableSurfUp: exclusionMask |= weaponModifiers.SurfUp

if args.enableQuickShot: inclusionMask |= weaponModifiers.QuickShot
if args.disableQuickShot: exclusionMask |= weaponModifiers.QuickShot

if args.enableZoom: inclusionMask |= weaponModifiers.Zoom
if args.disableZoom: exclusionMask |= weaponModifiers.Zoom

if args.enableMultiShot: inclusionMask |= weaponModifiers.MultiShot
if args.disableMultiShot: exclusionMask |= weaponModifiers.MultiShot

if args.enableLongThrow: inclusionMask |= weaponModifiers.LongThrow
if args.disableLongThrow: exclusionMask |= weaponModifiers.LongThrow

if args.enableCriticalHit: inclusionMask |= weaponModifiers.CriticalHit
if args.disableCriticalHit: exclusionMask |= weaponModifiers.CriticalHit

if args.enableDurabilityUp: inclusionMask |= weaponModifiers.DurabilityUp
if args.disableDurabilityUp: exclusionMask |= weaponModifiers.DurabilityUp

if args.enableAttackUp: inclusionMask |= weaponModifiers.AttackUp
if args.disableAttackUp: exclusionMask |= weaponModifiers.AttackUp

if args.excludeUnspecified:
    exclusionMask = 0b111111111 ^ inclusionMask

if inclusionMask == 0 and exclusionMask == 0 and args.recipes is None:
    parser.print_help()
    parser.exit()

recipeDataFile = open(args.recipeDataPath, "r")
recipeData = json.load(recipeDataFile)
recipeDataFile.close()

if not checkVersion(recipeData) == 2:
    print("\"" + str(args.recipeDataPath) + "\" is for the wrong version of this script. Please replace it (and if necessary, this script) with the latest version.")
    parser.exit()

materialData = recipeData[0]
cookData = recipeData[1]

NMMR = cookData['System']['NMMR']
for NMMRIndex in range(len(NMMR)):
    NMMR[NMMRIndex] = np.float32(NMMR[NMMRIndex])

numOfRecipes = args.numOfRecipes if args.numOfRecipes != 0 else None

if args.recipes is not None:
    results = []
    def checkResults():
        for resultRecipe in resultRecipes:
            buyTotal, sellTotal = 0, 0
            for ingredientIndex in resultRecipe['recipeIndexes']:
                material = materialData[ingredientIndex]
                if "CookLowPrice" in material['Tags']:
                    buyTotal += 1
                    sellTotal += 1
                else:
                    buyTotal += material['BuyingPrice']
                    sellTotal += material['SellingPrice']
            price = getPrice(sellTotal, buyTotal, len(resultRecipe['recipe']))
            valWithCrit = resultRecipe['cookValue']
            if resultRecipe['critChance'] < 100 or (resultRecipe['effectType'] != "None" and resultRecipe['effectType'] != False): valWithCrit += 12
            if valWithCrit > 120: valWithCrit = 120
            if resultRecipe['effectType'] != "LifeMaxUp" and ((inclusionMask == 0 and exclusionMask == 0) or (inclusionMask & price == inclusionMask and exclusionMask & price == 0)) and ((not args.zeroVal and valWithCrit >= args.minVal) or (args.zeroVal and resultRecipe['cookValue'] == 0)):
                resultRecipe['recipeIndexes'].sort()
                results.append((valWithCrit, resultRecipe['cookValue'], price, resultRecipe['recipe'],resultRecipe['recipeName'], resultRecipe['recipeIndexes']))
                if numOfRecipes is not None and len(results) >= numOfRecipes: break
        else:
            return False
        return True
    maxResults = False
    for recipe in args.recipes:
        (materialsList, materialDifferGroups, materialSameGroups) = parseRecipe(recipe)
        def checkDifferGroup(materialIndexes):
            material = materialsList[len(materialIndexes) - 1][materialIndexes[-1]]
            for slotNum, materialIndex in enumerate(materialIndexes[:-1]):
                if materialDifferGroups[slotNum] == materialDifferGroups[len(materialIndexes) - 1] and materialsList[slotNum][materialIndex] == material : return True
            return False
        def checkSameGroup(materialIndexes):
            material = materialsList[len(materialIndexes) - 1][materialIndexes[-1]]
            for slotNum, materialIndex in enumerate(materialIndexes[:-1]):
                if materialSameGroups[slotNum] == materialSameGroups[len(materialIndexes) - 1] and materialsList[slotNum][materialIndex] != material : return True
            return False
        checkRecipeCount, numIterations = 0, 1
        for material in materialsList:
            numIterations *= len(material)
        for material1Index in range(len(materialsList[0])):
            if maxResults: break
            if len(materialsList) == 1:
                checkRecipeCount += 1
                if checkRecipeCount % 10000 == 0: print(f'{str(int(checkRecipeCount / numIterations * 100)) + "%": >4}', file=sys.stderr, end='\r')
            if len(materialsList) > 1:
                for material2Index in range(len(materialsList[1])):
                    if maxResults: break
                    if len(materialsList) == 2:
                        checkRecipeCount += 1
                        if checkRecipeCount % 10000 == 0: print(f'{str(int(checkRecipeCount / numIterations * 100)) + "%": >4}', file=sys.stderr, end='\r')
                    if materialDifferGroups[1] is not None and checkDifferGroup([material1Index, material2Index]):
                        skipIterations = 1
                        for material in materialsList[2:]:
                            skipIterations *= len(material)
                        checkRecipeCount += skipIterations
                        continue
                    if materialSameGroups[1] is not None and checkSameGroup([material1Index, material2Index]):
                        skipIterations = 1
                        for material in materialsList[2:]:
                            skipIterations *= len(material)
                        checkRecipeCount += skipIterations
                        continue
                    if len(materialsList) > 2:
                        for material3Index in range(len(materialsList[2])):
                            if maxResults: break
                            if len(materialsList) == 3:
                                checkRecipeCount += 1
                                if checkRecipeCount % 10000 == 0: print(f'{str(int(checkRecipeCount / numIterations * 100)) + "%": >4}', file=sys.stderr, end='\r')
                            if materialDifferGroups[2] is not None and checkDifferGroup([material1Index, material2Index, material3Index]):
                                skipIterations = 1
                                for material in materialsList[3:]:
                                    skipIterations *= len(material)
                                checkRecipeCount += skipIterations
                                continue
                            if materialSameGroups[2] is not None and checkSameGroup([material1Index, material2Index, material3Index]):
                                skipIterations = 1
                                for material in materialsList[3:]:
                                    skipIterations *= len(material)
                                checkRecipeCount += skipIterations
                                continue
                            if len(materialsList) > 3:
                                for material4Index in range(len(materialsList[3])):
                                    if maxResults: break
                                    if len(materialsList) == 4:
                                        checkRecipeCount += 1
                                        if checkRecipeCount % 10000 == 0: print(f'{str(int(checkRecipeCount / numIterations * 100)) + "%": >4}', file=sys.stderr, end='\r')
                                    if materialDifferGroups[3] is not None and checkDifferGroup([material1Index, material2Index, material3Index, material4Index]):
                                        skipIterations = len(materialsList[4])
                                        checkRecipeCount += skipIterations
                                        continue
                                    if materialSameGroups[3] is not None and checkSameGroup([material1Index, material2Index, material3Index, material4Index]):
                                        skipIterations = len(materialsList[4])
                                        checkRecipeCount += skipIterations
                                        continue
                                    if len(materialsList) > 4:
                                        for material5Index in range(len(materialsList[4])):
                                            if maxResults: break
                                            if len(materialsList) == 5:
                                                checkRecipeCount += 1
                                                if checkRecipeCount % 10000 == 0: print(f'{str(int(checkRecipeCount / numIterations * 100)) + "%": >4}', file=sys.stderr, end='\r')
                                            if materialDifferGroups[4] is not None and checkDifferGroup([material1Index, material2Index, material3Index, material4Index, material5Index]): continue
                                            if materialSameGroups[4] is not None and checkSameGroup([material1Index, material2Index, material3Index, material4Index, material5Index]): continue
                                            resultRecipes = verifyRecipe([[materialsList[0][material1Index]],[materialsList[1][material2Index]],[materialsList[2][material3Index]],[materialsList[3][material4Index]],[materialsList[4][material5Index]]])
                                            if maxResults := checkResults(): break
                                    else:
                                        resultRecipes = verifyRecipe([[materialsList[0][material1Index]],[materialsList[1][material2Index]],[materialsList[2][material3Index]],[materialsList[3][material4Index]]])
                                        if maxResults := checkResults(): break
                            else:
                                resultRecipes = verifyRecipe([[materialsList[0][material1Index]],[materialsList[1][material2Index]],[materialsList[2][material3Index]]])
                                if maxResults := checkResults(): break
                    else:
                        resultRecipes = verifyRecipe([[materialsList[0][material1Index]],[materialsList[1][material2Index]]])
                        if maxResults := checkResults(): break
            else:
                resultRecipes = verifyRecipe([[materialsList[0][material1Index]]])
                if maxResults := checkResults(): break
    results.sort(key=lambda recipe: recipe[5])
    results = [result for resultCount, result in enumerate(results) if resultCount == 0 or results[resultCount - 1][5] != result[5]]
else:
    results = findRecipes(inclusionMask, exclusionMask, args.cookedFoods, args.minVal, args.zeroVal, args.allowElixirs, numOfRecipes, args.ingredientsToUse, args.ingredientsNotToUse)

if inclusionMask == 0 and exclusionMask == 0 and not args.zeroVal and args.minVal == 0:
    print("Processed " + str(len(results)) + " recipes", file=sys.stderr)
else:
    print("Found " + str(len(results)) + " recipes", file=sys.stderr)

if args.csvOutPath is None:
    if inclusionMask == 0 and exclusionMask == 0:
        csvOutPath = "BOTW WMC Recipes.csv"
    else:
        csvOutPath = "BOTW WMC Recipes ["
        includeModifierList = []
        for flag in weaponModifiers:
            if flag & inclusionMask != 0: includeModifierList.append(flag.name)
        if len(includeModifierList) > 0: csvOutPath += "+"
        csvOutPath += ", +".join(includeModifierList)
        excludeModifierList = []
        for flag in weaponModifiers:
            if flag & exclusionMask != 0: excludeModifierList.append(flag.name)
        if len(includeModifierList) > 0 and len(excludeModifierList) > 0: csvOutPath += ", "
        if len(excludeModifierList) > 0: csvOutPath += "-"
        csvOutPath += ", -".join(excludeModifierList)
        csvOutPath += "].csv"
else:
    csvOutPath = args.csvOutPath

if len(results) > 0:
    print("Saving results to \"" + csvOutPath + "\"", file=sys.stderr)
    
    csvOutFile = open(csvOutPath, "w")
    
    results.sort(reverse=True)
    print("Recipe Name,Recipe,Value (With Crit),Value (Without Crit),Sell Price,Guard Up,Surf Up,Quick Shot,Zoom,Muti-Shot,Long Throw,Critical Hit,Durability Up,Attack Up", file=csvOutFile)
    recipeCount = 0
    for recipe in results:
        if numOfRecipes is not None and recipeCount >= numOfRecipes: break
        modifiersList = []
        for flag in weaponModifiers:
            modifiersList.append("Y" if flag & recipe[2] != 0 else "N")
        recipeMaterials = recipe[3]
        recipeMaterials.sort()
        print(recipe[4] + ",\"" + str(", ".join(recipeMaterials)) + "\"," + str(recipe[0]) + "," + str(recipe[1]) + "," + str(recipe[2]) + "," + ",".join(modifiersList), file=csvOutFile)
        recipeCount += 1
    csvOutFile.close()