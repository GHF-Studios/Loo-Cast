using System;
using System.Numerics;

namespace LooCast.System
{
    public struct BigFloat
    {
        #region Static Fields
        public static readonly BigFloat Zero = new BigFloat(0);
        public static readonly BigFloat One = new BigFloat(1);
        public static readonly BigFloat MinusOne = new BigFloat(-1);
        public static readonly BigFloat E = new BigFloat(MathF.E);
        public static readonly BigFloat PI = new BigFloat(MathF.PI);
        #endregion

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
            double log10 = (double)Log10(absValue);
            exponent = (int)Floor(log10);
            double scale = Pow(10, -exponent);
            mantissa = (BigInteger)Round(absValue * scale);

            if (value < 0)
            {
                mantissa = -mantissa;
            }
        }

        public BigFloat(float value)
        {
            float absValue = Abs(value);
            float log10 = (float)Log10(absValue);
            exponent = (int)Floor(log10);
            float scale = Pow(10, -exponent);
            mantissa = (BigInteger)Round(absValue * scale);

            if (value < 0)
            {
                mantissa = -mantissa;
            }
        }

        public BigFloat(int value)
        {
            mantissa = value;
            exponent = 0;
        }
        #endregion

        #region Static Methods
        public static BigFloat Abs(BigFloat value)
        {
            return new BigFloat(BigInteger.Abs(value.mantissa), value.exponent);
        }

        public static BigFloat Log(BigFloat value)
        {
            if (value <= 0)
            {
                throw new ArgumentException("Logarithm of a non-positive number is undefined!");
            }

            if (value == 1)
            {
                return 0;
            }

            double log2 = Log2(value.ToDouble());
            double log10 = log2 * 3.32192809488736218170856773213; // log10(2) = 3.32192809488736218170856773213
            int exponent = (int)Floor(log10);
            double scale = Pow(10, -exponent);
            BigInteger mantissa = (BigInteger)Round(log10 * scale);

            return new BigFloat(mantissa, exponent);
        }

        public static BigFloat Log10(BigFloat value)
        {
            return Log(value) / Log(10);
        }

        public static BigFloat Log2(BigFloat value)
        {
            return Log10(value) / Log10(2);
        }

        public static BigFloat Pow(BigFloat value, BigFloat exponent)
        {
            if (value == 0)
            {
                if (exponent == 0)
                {
                    throw new ArgumentException("0^0 is undefined!");
                }
                else
                {
                    return 0;
                }
            }

            if (exponent == 0)
            {
                return 1;
            }

            if (value == 1 || exponent == 1)
            {
                return value;
            }

            if (value == -1)
            {
                if (exponent % 2 == 0)
                {
                    return 1;
                }
                else
                {
                    return -1;
                }
            }

            if (exponent < 0)
            {
                return Pow(1 / value, -exponent);
            }

            BigInteger resultMantissa = BigInteger.Pow(value.mantissa, exponent);
            int resultExponent = value.exponent * exponent;

            while (resultMantissa % 10 == 0 && resultExponent > int.MinValue)
            {
                resultMantissa /= 10;
                resultExponent--;
            }

            return new BigFloat(resultMantissa, resultExponent);
        }

        public static BigFloat Floor(BigFloat value)
        {
            if (value.mantissa >= 0)
            {
                return new BigFloat(value.mantissa / BigInteger.Pow(10, value.exponent), 0);
            }
            else
            {
                BigInteger quotient = BigInteger.DivRem(value.mantissa, BigInteger.Pow(10, value.exponent), out BigInteger remainder);
                if (remainder != 0)
                {
                    quotient -= 1;
                }
                return new BigFloat(quotient, 0);
            }
        }

        public static BigFloat Ceil(BigFloat value)
        {
            if (value.mantissa >= 0)
            {
                BigInteger quotient = BigInteger.DivRem(value.mantissa, BigInteger.Pow(10, value.exponent), out BigInteger remainder);
                if (remainder != 0)
                {
                    quotient += 1;
                }
                return new BigFloat(quotient, 0);
            }
            else
            {
                return new BigFloat(value.mantissa / BigInteger.Pow(10, value.exponent), 0);
            }
        }

        public static BigFloat Round(BigFloat value, int digits = 0)
        {
            if (digits < 0)
            {
                throw new ArgumentOutOfRangeException(nameof(digits), "Number of digits cannot be negative.");
            }

            BigInteger scale = BigInteger.Pow(10, digits);
            BigInteger half = scale / 2;
            BigInteger absMantissa = BigInteger.Abs(value.mantissa);

            if (absMantissa % scale >= half)
            {
                BigInteger roundUp = value.mantissa > 0 ? scale : -scale;
                value.mantissa = value.mantissa + roundUp;
            }

            value.exponent -= digits;

            while (value.mantissa % 10 == 0 && value.exponent > int.MinValue)
            {
                value.mantissa /= 10;
                value.exponent--;
            }

            return value;
        }

        public static BigInteger FloorToBigInt(BigFloat value)
        {
            BigInteger floor = BigInteger.Divide(value.mantissa, BigInteger.Pow(10, -value.exponent));
            if (value.mantissa < 0 && value.exponent < 0 && value.mantissa % BigInteger.Pow(10, -value.exponent) != 0)
            {
                floor -= 1;
            }
            return floor;
        }

        public static BigInteger CeilToBigInt(BigFloat value)
        {
            if (value.exponent >= 0)
            {
                return value.mantissa * BigInteger.Pow(10, value.exponent);
            }
            else
            {
                BigInteger scale = BigInteger.Pow(10, -value.exponent);
                BigInteger ceiling = value.mantissa / scale;
                if (value.mantissa % scale != 0)
                {
                    ceiling += 1;
                }
                return ceiling;
            }
        }

        public static BigInteger RoundToBigInt(BigInteger value, int digits = 0)
        {
            if (digits < 0)
            {
                throw new ArgumentException("Number of digits cannot be negative", nameof(digits));
            }

            if (digits == 0)
            {
                return value;
            }

            BigInteger scale = BigInteger.Pow(10, digits);
            BigInteger half = (value.Sign < 0) ? -scale / 2 : scale / 2;
            BigInteger rounded = (value + half) / scale;

            return rounded * scale;
        }

        public static BigFloat Root(BigFloat value, int n)
        {
            if (n == 0)
            {
                throw new ArgumentException("Root degree cannot be zero.");
            }

            if (value < 0 && n % 2 == 0)
            {
                throw new ArgumentException("Cannot take even root of a negative number.");
            }

            if (value == 0)
            {
                return 0;
            }

            BigInteger resultMantissa = BigInteger.Pow(BigInteger.Abs(value.mantissa), (int)(1.0 / n));
            int resultExponent = value.exponent / n;

            // Adjust mantissa and exponent to account for rounding errors
            while (BigInteger.Pow(resultMantissa, n) > BigInteger.Abs(value.mantissa))
            {
                resultMantissa--;
            }

            while (BigInteger.Pow(resultMantissa + 1, n) <= BigInteger.Abs(value.mantissa))
            {
                resultMantissa++;
            }

            if (value.mantissa < 0 && n % 2 == 1)
            {
                resultMantissa = -resultMantissa;
            }

            return new BigFloat(resultMantissa, resultExponent);
        }

        public static BigFloat Sqrt(BigFloat value)
        {
            return Root(value, 2);
        }

        public static BigFloat Sin(BigFloat angle)
        {
            double angleInRadians = angle.ToDouble() * PI / 180;
            return new BigFloat(Sin(angleInRadians));
        }

        public static BigFloat Cos(BigFloat angle)
        {
            double angleInRadians = angle.ToDouble() * PI / 180;
            return new BigFloat(Cos(angleInRadians));
        }

        public static BigFloat Tan(BigFloat angle)
        {
            double angleInRadians = angle.ToDouble() * PI / 180;
            return new BigFloat(Tan(angleInRadians));
        }

        public static BigFloat Csc(BigFloat angle)
        {
            return 1 / Sin(angle);
        }

        public static BigFloat Sec(BigFloat angle)
        {
            return 1 / Cos(angle);
        }

        public static BigFloat Cot(BigFloat angle)
        {
            return 1 / Tan(angle);
        }

        public static BigFloat Asin(BigFloat value)
        {
            double asinInRadians = Asin(value.ToDouble());
            return new BigFloat(asinInRadians * 180 / PI);
        }

        public static BigFloat Acos(BigFloat value)
        {
            double acosInRadians = Acos(value.ToDouble());
            return new BigFloat(acosInRadians * 180 / PI);
        }

        public static BigFloat Atan(BigFloat value)
        {
            double atanInRadians = Atan(value.ToDouble());
            return new BigFloat(atanInRadians * 180 / PI);
        }

        public static BigFloat Acsc(BigFloat value)
        {
            return Asin(1 / value);
        }

        public static BigFloat Asec(BigFloat value)
        {
            return Acos(1 / value);
        }

        public static BigFloat Acot(BigFloat value)
        {
            return Atan(1 / value);
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
        public static implicit operator double(BigFloat value)
        {
            if (value.exponent > 308)
            {
                throw new OverflowException("The value is too large to fit in a double!");
            }
            return value.ToDouble();
        }

        public static implicit operator float(BigFloat value)
        {
            if (value.exponent > 38)
            {
                throw new OverflowException("The value is too large to fit in a float!");
            }
            return (float)value.ToDouble();
        }

        public static implicit operator int(BigFloat value)
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

        public static implicit operator BigFloat(BigInteger value)
        {
            return new BigFloat(value, 0);
        }

        public static implicit operator BigInteger(BigFloat value)
        {
            return value.mantissa * BigInteger.Pow(10, value.exponent);
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