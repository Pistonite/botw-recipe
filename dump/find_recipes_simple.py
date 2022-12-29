### Modified by iTNTPiston
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
from enum import IntFlag
import sys
import re

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

def parseIngredient(ingredient):
    return [ingredient]

def parseRecipe(recipe, materialData):
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

def getPrice(NMMR, sellTotal, buyTotal, ingredientCount):
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

def verifyRecipe(cookData, materialData, materialsList: list):
    effectType, cookValue, critChance, numUnique, recipe, recipeList, recipeTags, ingredientIndexes = False, 0, 0, 0, [], [], [], []
    materialsList = materialsList.copy()
    dubiousValue = 0
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
        if material['EffectType'] == "LifeMaxUp":
            dubiousValue += material['HitPointRecover'] + 4
        cookValue += material['HitPointRecover'] * 2
        if materialsList.index(materialsList[materialNameIndex]) == materialNameIndex:
            cookValue += material['BoostHitPointRecover']
            critChance += material['BoostSuccessRate']
            numUnique += 1
        recipeTags.append(material['Tags'])
    if cookValue > 120: cookValue = 120
    if critChance > 100: critChance = 100
    if dubiousValue < 4: dubiousValue = 4
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
            if checkRecipe['Recipe'] != "Fairy Tonic":
                recipeName = addRecipePrefix(checkRecipe['Recipe'], effectType)
            else:
                recipeName = checkRecipe['Recipe']
            if recipeName == "Elixir": 
                recipeName = "Dubious Food"
            if recipeName == "Rock-Hard Food":
                cookValue = 1
            elif recipeName == "Dubious Food":
                cookValue = dubiousValue
            
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
        if checkRecipe['Recipe'] != "Fairy Tonic" and checkRecipe['Recipe'] != "Dubious Food" and checkRecipe['Recipe'] != "Rock-Hard Food":
            recipeName = addRecipePrefix(checkRecipe['Recipe'], effectType)
        else:
            recipeName = checkRecipe['Recipe']
        if recipeName == "Elixir": 
            recipeName = "Dubious Food"
        if recipeName == "Rock-Hard Food":
            cookValue = 1
        elif recipeName == "Dubious Food":
            cookValue = dubiousValue
        
        if 'HB' in checkRecipe: cookValue += int(checkRecipe['HB'])
        if critChance == 100 and (effectType == "None" or effectType == False):
            cookValue += 12
        if cookValue > 120: cookValue = 120
        rtv = [{'recipeName': recipeName, 'recipe': recipe, 'recipeIndexes': ingredientIndexes, 'cookValue': cookValue, 'critChance': critChance, 'effectType': effectType}]
        #print(rtv)
        return rtv
    return [{'recipeName': "Dubious Food", 'recipe': recipe, 'recipeIndexes': ingredientIndexes, 'cookValue': dubiousValue, 'critChance': 0, 'effectType': "None"}]

def process_recipe(recipe_data, recipe_str):

    recipeData = recipe_data
    materialData = recipeData[0]
    cookData = recipeData[1]

    NMMR = cookData['System']['NMMR']
    for NMMRIndex in range(len(NMMR)):
        NMMR[NMMRIndex] = np.float32(NMMR[NMMRIndex])

    results = []
    def checkResults(resultRecipes):
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
            if resultRecipe['recipeName'] != "Fairy Tonic" and resultRecipe['recipeName'] != "Dubious Food" and resultRecipe['recipeName'] != "Rock-Hard Food":
                price = getPrice(NMMR, sellTotal, buyTotal, len(resultRecipe['recipe']))
            else:
                price = 2
            #if resultRecipe['recipeName'] == "Dubious Food":
            #    resultRecipe['cookValue']
            if resultRecipe['recipeName'] != "Dubious Food" and resultRecipe['recipeName'] != "Rock-Hard Food":
                valWithCrit = resultRecipe['cookValue']
                if resultRecipe['critChance'] < 100 or (resultRecipe['effectType'] != "None" and resultRecipe['effectType'] != False): valWithCrit += 12
                if valWithCrit > 120: valWithCrit = 120
            else:
                valWithCrit = resultRecipe['cookValue']
            
            results.append((valWithCrit, resultRecipe['cookValue'], price, resultRecipe['recipe'],resultRecipe['recipeName'], resultRecipe['recipeIndexes']))

    (materialsList, materialDifferGroups, materialSameGroups) = parseRecipe(recipe_str, materialData)


    for l in materialsList:
        if len(l) != 1:
            raise ValueError(f"length not 1: {l} for input str {recipe_str}")


    resultRecipes = verifyRecipe(
        cookData, 
        materialData, 
        materialsList)
    if not resultRecipes:
        raise ValueError(f"resultRecipes is empty for ")
    #print(resultRecipes)
    checkResults(resultRecipes)

    
    if len(results) != 1:
        raise ValueError(f"Uhhhh length {len(results)} for {materialsList} for {recipe_str} but early")
    results = [result for resultCount, result in enumerate(results) if resultCount == 0 or results[resultCount - 1][5] != result[5]]
    if len(results) != 1:
        raise ValueError(f"Uhhhh length {len(results)} for {materialsList} for {recipe_str}")
    recipe = results[0]
   
    return recipe[0], recipe[1], recipe[2]

if __name__ == "__main__":
    with open("recipeData.json", "r", encoding="utf-8") as recipe_file:
        recipe_data = json.load(recipe_file)
    print(process_recipe(recipe_data, sys.argv[1]))
