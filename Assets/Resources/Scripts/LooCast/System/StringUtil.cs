using System;
using System.Linq;
using System.Collections.Generic;

namespace LooCast.System
{
    public static class StringUtil
    {
        public static bool IsAlphaNumeric(string stringValue)
        {
            return stringValue.All(char.IsLetterOrDigit);
        }
        
        public static bool IsAlphaNumericWithExceptions(string stringValue, params char[] exceptions)
        {
            return stringValue.All(c => char.IsLetterOrDigit(c) || exceptions.Contains(c));
        }

        public static bool IsEmpty(string stringValue)
        {
            return string.IsNullOrEmpty(stringValue) || string.IsNullOrWhiteSpace(stringValue);
        }
    }
}
