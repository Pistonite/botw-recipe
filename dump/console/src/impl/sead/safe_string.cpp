#include <prim/seadSafeString.h>
#include <prim/seadStringUtil.h>

namespace
{
static const char16 cEmptyStringChar16[1] = u"";

}  // namespace

namespace sead
{
template <>
const char SafeStringBase<char>::cNullChar = '\0';

template <>
const char SafeStringBase<char>::cLineBreakChar = '\n';

template <>
const SafeStringBase<char> SafeStringBase<char>::cEmptyString("");

template <>
const char16 SafeStringBase<char16>::cNullChar = 0;

template <>
const char16 SafeStringBase<char16>::cLineBreakChar = static_cast<char16>('\n');

template <>
const SafeStringBase<char16> SafeStringBase<char16>::cEmptyString(cEmptyStringChar16);

template <>
SafeStringBase<char>& SafeStringBase<char>::operator=(const SafeStringBase<char>& other) = default;

template <>
SafeStringBase<char16>&
SafeStringBase<char16>::operator=(const SafeStringBase<char16>& other) = default;
}
