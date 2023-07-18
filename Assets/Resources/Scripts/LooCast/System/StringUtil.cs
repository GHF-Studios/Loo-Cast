using System;
using System.Linq;

namespace LooCast.System
{
    public static class StringUtil
    {
        public static bool IsAlphaNumeric(string stringValue)
        {
            if (stringValue is null)
            {
                return false;
            }
            
            return stringValue.All(char.IsLetterOrDigit);
        }
        
        public static bool IsAlphaNumericWithExceptions(string stringValue, params char[] exceptions)
        {
            if (stringValue is null)
            {
                return false;
            }

            return stringValue.All(c => char.IsLetterOrDigit(c) || exceptions.Contains(c));
        }

        public static bool IsEmpty(string stringValue)
        {
            return string.IsNullOrEmpty(stringValue) || string.IsNullOrWhiteSpace(stringValue);
        }

        public static bool IsNumeric(string stringValue)
        {
            if (stringValue is null)
            {
                return false;
            }

            return stringValue.All(char.IsDigit);
        }
    }
    }
}
