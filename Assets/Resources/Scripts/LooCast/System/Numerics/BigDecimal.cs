using System;
using System.Numerics;
using System.Globalization;

namespace LooCast.System.Numerics
{
    public struct BigDecimal
    {
        #region Static Fields
        public static readonly BigDecimal FloatMin = float.MinValue;
        public static readonly BigDecimal FloatMax = float.MaxValue;
        public static readonly BigDecimal DoubleMin = double.MinValue;
        public static readonly BigDecimal DoubleMax = double.MaxValue;
        public static readonly BigDecimal IntMin = int.MinValue;
        public static readonly BigDecimal IntMax = int.MaxValue;

        public static readonly BigDecimal Zero = new BigDecimal(0, 0);
        public static readonly BigDecimal One = new BigDecimal(1, 0);
        #endregion

        #region Properties
        public BigInteger Mantissa { get; set; }
        public int Exponent { get; set; }
        #endregion

        #region Constructors
        public BigDecimal(BigInteger mantissa, int exponent)
        {
            Mantissa = mantissa;
            Exponent = exponent;

            Normalize();
        }
        #endregion

        #region Static Methods
        public static BigDecimal FromIntegerExponentialString(string value)
        {
            string[] parts = value.Split(new string[] { "x10^", "*10^" }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                throw new Exception($"Integer exponential notation '{value}' is invalid!");
            }
            if (!StringUtil.IsNumericWithExceptions(parts[0], '-') || !StringUtil.IsNumericWithExceptions(parts[1], '-'))
            {
                throw new Exception($"Integer exponential notation '{value}' is invalid!");
            }
            if (!BigInteger.TryParse(parts[0], out BigInteger mantissa))
            {
                throw new Exception($"Integer exponential notation '{value}' is invalid! The mantissa '{parts[0]}' could not be parsed to a BigInteger!");
            }
            if (!int.TryParse(parts[1], out int exponent))
            {
                throw new Exception($"Integer exponential notation '{value}' is invalid! The exponent '{parts[1]}' could not be parsed to an int!");
            }

            return new BigDecimal(mantissa, exponent);
        }

        public static BigDecimal FromDecimalString(string value)
        {
            string[] parts = value.Split(".", StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length == 0)
            {
                throw new Exception($"Decimal notation '{value}' is invalid!");
            }

            BigInteger mantissa;
            int exponent;

            if (parts.Length == 1)
            {
                if (!BigInteger.TryParse(parts[0], out mantissa))
                {
                    throw new Exception($"Decimal notation '{value}' is invalid! The mantissa '{parts[0]}' could not be parsed to a BigInteger!");
                }
                exponent = 0;
            }
            else if (parts.Length == 2)
            {
                if (!BigInteger.TryParse(parts[0] + parts[1], out mantissa))
                {
                    throw new Exception($"Decimal notation '{value}' is invalid! The mantissa '{parts[0] + parts[1]}' could not be parsed to a BigInteger!");
                }
                exponent = -parts[1].Length;
            }
            else
            {
                throw new Exception($"Decimal notation '{value}' is invalid!");
            }

            return new BigDecimal(mantissa, exponent);
        }

        public static BigDecimal FromScientificString(string value)
        {
            string[] parts = value.Split(new char[] { 'E', 'e' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                throw new Exception($"Scientific notation '{value}' is invalid!");
            }
            if (!StringUtil.IsNumericWithExceptions(parts[0], new char[] { '-', '+', '.' }) || !StringUtil.IsNumericWithExceptions(parts[1], new char[] { '-', '+', '.' }))
            {
                throw new Exception($"Scientific notation '{value}' is invalid!");
            }

            string mantissaString = parts[0];
            string exponentString = parts[1];

            mantissaString = mantissaString.Trim();
            exponentString = exponentString.Trim();

            int exponent;
            BigInteger mantissa;

            int exponentShift = 0;
            int decimalIndex = mantissaString.IndexOf('.');
            if (decimalIndex != -1)
            {
                exponentShift = -(mantissaString.Length - 1 - decimalIndex);
                mantissaString = mantissaString.Replace(".", "");
            }

            bool isNegative = mantissaString.StartsWith("-");
            if (isNegative)
            {
                mantissaString = mantissaString.Substring(1);
            }

            if (!BigInteger.TryParse(mantissaString, out mantissa))
            {
                throw new Exception($"Scientific notation '{value}' is invalid! The mantissa '{mantissaString}' could not be parsed to a BigInteger!");
            }
            if (!int.TryParse(exponentString, out exponent))
            {
                throw new Exception($"Scientific notation '{value}' is invalid! The exponent '{exponentString}' could not be parsed to an int!");
            }

            if (isNegative)
            {
                mantissa = BigInteger.Negate(mantissa);
            }

            exponent += exponentShift;

            return new BigDecimal(mantissa, exponent);
        }

        public static BigDecimal Abs(BigDecimal value)
        {
            return new BigDecimal(BigInteger.Abs(value.Mantissa), value.Exponent);
        }
        /*
        public static BigDecimal Pow(BigDecimal value, BigInteger exponent)
        {
            
        }

        public static BigDecimal Sqrt(BigDecimal value)
        {

        }

        public static BigDecimal Floor(BigDecimal value)
        {

        }

        public static BigDecimal Ceil(BigDecimal value)
        {

        }
        */
        private static int Compare(BigDecimal left, BigDecimal right)
        {
            if (left.Exponent != right.Exponent)
            {
                ScaleToEqualExponent(ref left, ref right);
            }

            return BigInteger.Compare(left.Mantissa, right.Mantissa);
        }

        private static void ScaleToEqualExponent(ref BigDecimal bigDecimal1, ref BigDecimal bigDecimal2)
        {
            if (bigDecimal1.Exponent > bigDecimal2.Exponent)
            {
                int difference = bigDecimal1.Exponent - bigDecimal2.Exponent;
                bigDecimal1.Mantissa *= BigInteger.Pow(10, difference);
                bigDecimal1.Exponent -= difference;
            }
            else if (bigDecimal2.Exponent > bigDecimal1.Exponent)
            {
                int difference = bigDecimal2.Exponent - bigDecimal1.Exponent;
                bigDecimal2.Mantissa *= BigInteger.Pow(10, difference);
                bigDecimal2.Exponent -= difference;
            }
        }

        #endregion

        #region Methods
        public string ToIntegerExponentialString()
        {
            Normalize();

            return $"{Mantissa}x10^{Exponent}";
        }

        public string ToDecimalString()
        {
            Normalize();

            string mantissaString = Mantissa.ToString();
            bool isNegative = mantissaString.StartsWith("-");
            if (isNegative)
            {
                mantissaString = mantissaString.Substring(1);
            }

            if (Exponent < 0)
            {
                int decimalPlaces = Math.Abs(Exponent);
                if (mantissaString.Length <= decimalPlaces)
                {
                    mantissaString = mantissaString.PadLeft(decimalPlaces + 1, '0');
                }
                mantissaString = mantissaString.Insert(mantissaString.Length - decimalPlaces, ".");
            }
            else if (Exponent > 0)
            {
                mantissaString = mantissaString.PadRight(mantissaString.Length + Exponent, '0');
            }

            if (isNegative)
            {
                mantissaString = mantissaString.Insert(0, "-");
            }

            return mantissaString;
        }

        public string ToScientificString()
        {
            Normalize();

            string mantissaString = Mantissa.ToString();
            bool isNegative = mantissaString.StartsWith("-");
            if (isNegative)
            {
                mantissaString = mantissaString.Substring(1);
            }

            int adjustedExponent = Exponent;

            if (mantissaString.Length > 1)
            {
                adjustedExponent += mantissaString.Length - 1;
                mantissaString = mantissaString.Insert(1, ".");
            }
            else
            {
                mantissaString += ".0";
            }

            string exponentString = adjustedExponent.ToString();

            return $"{(isNegative ? "-" : "")}{mantissaString}E{exponentString}";
        }

        private void Normalize()
        {
            if (Mantissa.IsZero)
            {
                Exponent = 0;
                return;
            }

            BigInteger remainder;
            BigInteger dividedMantissa = BigInteger.DivRem(Mantissa, 10, out remainder);
            int trailingZeros = 0;

            while (remainder.IsZero)
            {
                Mantissa = dividedMantissa;
                dividedMantissa = BigInteger.DivRem(Mantissa, 10, out remainder);
                trailingZeros++;
            }

            if (trailingZeros > 0)
            {
                Mantissa = remainder;
                Exponent += trailingZeros;
            }
        }
        #endregion

        #region Overrides
        public bool Equals(BigDecimal other)
        {
            return Mantissa.Equals(other.Mantissa) && Exponent.Equals(other.Exponent);
        }

        public override bool Equals(object obj)
        {
            if (obj is not BigDecimal)
            {
                return false;
            }

            return Equals((BigDecimal)obj);
        }

        public override int GetHashCode()
        {
            unchecked
            {
                int hash = 17;
                hash = hash * 23 + Mantissa.GetHashCode();
                hash = hash * 23 + Exponent.GetHashCode();
                return hash;
            }
        }

        public override string ToString()
        {
            return ToDecimalString();
        }
        #endregion

        #region Operators
        public static explicit operator float(BigDecimal value)
        {
            if (value < FloatMin || value > FloatMax)
            {
                throw new OverflowException($"BigDecimal '{value}' is outside the range of a float!");
            }
            if (!float.TryParse(value.ToString(), out float result))
            {
                throw new Exception($"BigDecimal '{value}' could not be parsed to a float!");
            }

            return result;
        }

        public static explicit operator double(BigDecimal value)
        {
            if (value < DoubleMin || value > DoubleMax)
            {
                throw new OverflowException($"BigDecimal is outside the range of a double!");
            }
            if (!double.TryParse(value.ToString(), out double result))
            {
                throw new Exception($"BigDecimal '{value}' could not be parsed to a double!");
            }

            return result;
        }

        public static explicit operator int(BigDecimal value)
        {
            if (value < int.MinValue || value > int.MaxValue)
            {
                throw new OverflowException($"BigDecimal is outside the range of an int!");
            }
            if (!int.TryParse(value.ToString(), out int result))
            {
                throw new Exception($"BigDecimal '{value}' could not be parsed to an int!");
            }

            return result;
        }

        public static implicit operator BigDecimal(float value)
        {
            string stringValue = value.ToString(CultureInfo.InvariantCulture);
            if (stringValue.Contains('E'))
            {
                return FromScientificString(stringValue);
            }
            else
            {
                return FromDecimalString(stringValue);
            }
        }

        public static implicit operator BigDecimal(double value)
        {
            string stringValue = value.ToString(CultureInfo.InvariantCulture);
            if (stringValue.Contains('E'))
            {
                return FromScientificString(stringValue);
            }
            else
            {
                return FromDecimalString(stringValue);
            }
        }

        public static implicit operator BigDecimal(int value)
        {
            string stringValue = value.ToString(CultureInfo.InvariantCulture);
            return FromDecimalString(stringValue);
        }

        public static bool operator <(BigDecimal leftValue, BigDecimal rightValue)
        {
            return Compare(leftValue, rightValue) < 0;
        }

        public static bool operator <=(BigDecimal leftValue, BigDecimal rightValue)
        {
            return Compare(leftValue, rightValue) <= 0;
        }

        public static bool operator >(BigDecimal leftValue, BigDecimal rightValue)
        {
            return Compare(leftValue, rightValue) > 0;
        }

        public static bool operator >=(BigDecimal leftValue, BigDecimal rightValue)
        {
            return Compare(leftValue, rightValue) >= 0;
        }

        public static bool operator ==(BigDecimal leftValue, BigDecimal rightValue)
        {
            return Compare(leftValue, rightValue) == 0;
        }

        public static bool operator !=(BigDecimal leftValue, BigDecimal rightValue)
        {
            return Compare(leftValue, rightValue) != 0;
        }

        public static BigDecimal operator ++(BigDecimal value)
        {
            return value + One;
        }

        public static BigDecimal operator --(BigDecimal value)
        {
            return value - One;
        }

        public static BigDecimal operator +(BigDecimal value)
        {
            return value;
        }

        public static BigDecimal operator -(BigDecimal value)
        {
            return new BigDecimal(-value.Mantissa, value.Exponent);
        }

        public static BigDecimal operator +(BigDecimal leftValue, BigDecimal rightValue)
        {
            if (leftValue.Exponent != rightValue.Exponent)
            {
                ScaleToEqualExponent(ref leftValue, ref rightValue);
            }

            BigDecimal result = new BigDecimal(leftValue.Mantissa + rightValue.Mantissa, leftValue.Exponent);
            result.Normalize();

            return result;
        }

        public static BigDecimal operator -(BigDecimal leftValue, BigDecimal rightValue)
        {
            if (leftValue.Exponent != rightValue.Exponent)
            {
                ScaleToEqualExponent(ref leftValue, ref rightValue);
            }

            BigDecimal result = new BigDecimal(leftValue.Mantissa - rightValue.Mantissa, leftValue.Exponent);
            result.Normalize();

            return result;
        }

        public static BigDecimal operator *(BigDecimal leftValue, BigDecimal rightValue)
        {
            BigInteger mantissa = leftValue.Mantissa * rightValue.Mantissa;
            int exponent = leftValue.Exponent + rightValue.Exponent;

            BigDecimal result = new BigDecimal(mantissa, exponent);
            result.Normalize();

            return result;
        }

        public static BigDecimal operator /(BigDecimal leftValue, BigDecimal rightValue)
        {
            ScaleToEqualExponent(ref leftValue, ref rightValue);

            BigInteger mantissa = leftValue.Mantissa / rightValue.Mantissa;
            int exponent = leftValue.Exponent - rightValue.Exponent;

            BigDecimal result = new BigDecimal(mantissa, exponent);
            result.Normalize();

            return result;
        }

        public static BigDecimal operator %(BigDecimal leftValue, BigDecimal rightValue)
        {
            ScaleToEqualExponent(ref leftValue, ref rightValue);

            BigInteger mantissa = leftValue.Mantissa % rightValue.Mantissa;
            int exponent = leftValue.Exponent;

            BigDecimal result = new BigDecimal(mantissa, exponent);
            result.Normalize();

            return result;
        }
        #endregion
    }
}
