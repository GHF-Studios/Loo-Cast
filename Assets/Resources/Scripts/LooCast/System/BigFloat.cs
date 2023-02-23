using System;
using System.Numerics;

namespace LooCast.System
{
    public struct BigFloat
    {
        #region Fields
        private BigInteger mantissa;
        private int exponent;
        #endregion

        #region Constructors
        public BigFloat(BigInteger mantissa, int exponent)
        {
            this.mantissa = mantissa;
            this.exponent = exponent;
        }

        public BigFloat(double value)
        {
            double absValue = Abs(value);
            double log10 = Log10(absValue);
            exponent = (int)Floor(log10);
            double scale = Pow(10, -exponent);
            mantissa = (BigInteger)Round(absValue * scale);

            if (value < 0)
            {
                mantissa = -mantissa;
            }
        }
        #endregion

        #region Static Methods
        public static BigFloat Log10(BigFloat value)
        {
            if (value <= 0)
            {
                throw new ArgumentException("Value must be positive.");
            }

            const int MaxIterations = 50;
            const int Precision = 200;

            BigFloat x = value / BigFloat.Pow(10, BigFloat.Floor(BigFloat.Log10(value)));

            BigFloat result = 0;
            for (int n = 0; n < MaxIterations; n++)
            {
                int k = 2 * n + 1;
                BigFloat numerator = BigFloat.Pow(x - 1, k) * BigFloat.Pow(x + 1, k);
                BigFloat denominator = k * BigFloat.Pow(2, k);
                BigFloat term = numerator / denominator;
                if (n % 2 == 0)
                {
                    result += term;
                }
                else
                {
                    result -= term;
                }
            }

            result *= 2 / BigFloat.Log(new BigFloat(10));

            result += BigFloat.Floor(BigFloat.Log10(value)) * new BigFloat(1);

            result.Round(Precision);

            return result;
        }

        public static BigFloat Pow(BigFloat value, BigFloat exponent)
        {
            if (exponent == 0)
            {
                return new BigFloat(1);
            }
            else if (exponent == 1)
            {
                return value;
            }
            else if (exponent < 0)
            {
                return 1 / Pow(value, -exponent);
            }
            else if (exponent % 1 == 0)
            {
                // Exponent is an integer
                BigFloat result = 1;
                for (int i = 0; i < exponent; i++)
                {
                    result *= value;
                }
                return result;
            }
            else
            {
                // Exponent is not an integer
                bool isNegative = false;
                if (value < 0)
                {
                    if (exponent % 2 != 0)
                    {
                        isNegative = true;
                    }
                    value = -value;
                }
                BigFloat result = Exp(exponent * Log(value));
                if (isNegative)
                {
                    result = -result;
                }
                return result;
            }
        }

        public static BigFloat Floor(BigFloat value)
        {
            return value.Sign < 0 ? -Ceil(-value) - 1 : new BigFloat((BigInteger)value);
        }

        public static BigFloat Ceil(BigFloat value)
        {
            
        }

        public static BigFloat Round(BigFloat value, int digits)
        {
            
        }

        public static BigFloat Log(BigFloat value)
        {
            
        }
        #endregion

        #region Methods
        private double ToDouble()
        {
            double scale = Pow(10, exponent);
            double value = (double)mantissa / scale;
            return value;
        }
        #endregion

        #region Operators
        public static explicit operator double(BigFloat value)
        {
            if (value.exponent > 308)
            {
                throw new OverflowException("The value is too large to fit in a double!");
            }
            return value.ToDouble();
        }

        public static explicit operator float(BigFloat value)
        {
            if (value.exponent > 38)
            {
                throw new OverflowException("The value is too large to fit in a float!");
            }
            return (float)value.ToDouble();
        }

        public static explicit operator int(BigFloat value)
        {
            if (value.exponent > 9)
            {
                throw new OverflowException("The value is too large to fit in an int!");
            }
            return (int)value.ToDouble();
        }

        public static implicit operator BigFloat(int value)
        {
            return new BigFloat(value);
        }

        public static implicit operator BigFloat(float value)
        {
            return new BigFloat(value);
        }

        public static implicit operator BigFloat(double value)
        {
            return new BigFloat(value);
        }

        public static BigFloat operator +(BigFloat left, BigFloat right)
        {
            // Align the exponents and add the mantissas
            int delta = left.exponent - right.exponent;
            BigInteger leftMantissa = left.mantissa * BigInteger.Pow(10, delta);
            BigInteger rightMantissa = right.mantissa;
            BigInteger resultMantissa = leftMantissa + rightMantissa;
            int resultExponent = left.exponent;

            // Normalize the result if necessary
            while (resultMantissa % 10 == 0 && resultExponent > int.MinValue)
            {
                resultMantissa /= 10;
                resultExponent--;
            }

            return new BigFloat(resultMantissa, resultExponent);
        }

        public static BigFloat operator -(BigFloat left, BigFloat right)
        {
            // Align the exponents and subtract the mantissas
            int delta = left.exponent - right.exponent;
            BigInteger leftMantissa = left.mantissa * BigInteger.Pow(10, delta);
            BigInteger rightMantissa = right.mantissa;
            BigInteger resultMantissa = leftMantissa - rightMantissa;
            int resultExponent = left.exponent;

            // Normalize the result if necessary
            while (resultMantissa % 10 == 0 && resultExponent > int.MinValue)
            {
                resultMantissa /= 10;
                resultExponent--;
            }

            return new BigFloat(resultMantissa, resultExponent);
        }

        public static BigFloat operator *(BigFloat left, BigFloat right)
        {
            BigInteger resultMantissa = left.mantissa * right.mantissa;
            int resultExponent = left.exponent + right.exponent;

            // Normalize the result if necessary
            while (resultMantissa % 10 == 0 && resultExponent > int.MinValue)
            {
                resultMantissa /= 10;
                resultExponent--;
            }

            return new BigFloat(resultMantissa, resultExponent);
        }

        public static BigFloat operator /(BigFloat left, BigFloat right)
        {
            BigInteger resultMantissa = left.mantissa * BigInteger.Pow(10, left.exponent - right.exponent) / right.mantissa;
            int resultExponent = left.exponent - right.exponent;

            // Normalize the result if necessary
            while (resultMantissa % 10 == 0 && resultExponent > int.MinValue)
            {
                resultMantissa /= 10;
                resultExponent--;
            }

            return new BigFloat(resultMantissa, resultExponent);
        }

        public static BigFloat operator -(BigFloat value)
        {
            BigInteger resultMantissa = -value.mantissa;
            int resultExponent = value.exponent;
            return new BigFloat(resultMantissa, resultExponent);
        }

        public static bool operator >(BigFloat left, BigFloat right)
        {
            return left.mantissa * BigInteger.Pow(10, left.exponent - right.exponent) > right.mantissa;
        }

        public static bool operator <(BigFloat left, BigFloat right)
        {
            return left.mantissa * BigInteger.Pow(10, left.exponent - right.exponent) < right.mantissa;
        }

        public static bool operator >=(BigFloat left, BigFloat right)
        {
            return left.mantissa * BigInteger.Pow(10, left.exponent - right.exponent) >= right.mantissa;
        }

        public static bool operator <=(BigFloat left, BigFloat right)
        {
            return left.mantissa * BigInteger.Pow(10, left.exponent - right.exponent) <= right.mantissa;
        }

        public static bool operator ==(BigFloat left, BigFloat right)
        {
            return left.mantissa == right.mantissa && left.exponent == right.exponent;
        }

        public static bool operator !=(BigFloat left, BigFloat right)
        {
            return left.mantissa != right.mantissa || left.exponent != right.exponent;
        }
        #endregion

        #region Overrides
        public override bool Equals(object obj)
        {
            if (obj is BigFloat)
            {
                return this == (BigFloat)obj;
            }
            else
            {
                return false;
            }
        }

        public override int GetHashCode()
        {
            return mantissa.GetHashCode() ^ exponent.GetHashCode();
        }

        public override string ToString()
        {
            return mantissa.ToString() + "E" + exponent.ToString();
        }
        #endregion
    }
}