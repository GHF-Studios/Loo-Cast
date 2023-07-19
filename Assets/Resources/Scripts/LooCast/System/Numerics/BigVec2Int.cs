using System.Numerics;
using UnityEngine;

namespace LooCast.System.Numerics
{
    public struct BigVec2Int
    {
        #region Static Properties
        public static readonly BigVec2Int Zero = new BigVec2Int(0, 0);
        public static readonly BigVec2Int One = new BigVec2Int(1, 1);
        #endregion

        #region Properties
        public BigInteger X { get; private set; }
        public BigInteger Y { get; private set; }
        #endregion

        #region Constructors
        public BigVec2Int(BigInteger x, BigInteger y)
        {
            X = x;
            Y = y;
        }
        #endregion

        #region Static Methods
        public static float Distance(BigVec2Int pointA, BigVec2Int pointB)
        {
            float deltaX = (float)(pointA.X - pointB.X);
            float deltaY = (float)(pointA.Y - pointB.Y);
            return Mathf.Sqrt(deltaX * deltaX + deltaY * deltaY);
        }
        #endregion

        #region Overrides
        public override string ToString()
        {
            return $"{X}_{Y}";
        }

        public override bool Equals(object obj)
        {
            if (obj is not BigVec2Int)
            {
                return false;
            }
            else
            {
                BigVec2Int bigVec2Int = (BigVec2Int)obj;
                return bigVec2Int.X == this.X && bigVec2Int.Y == this.Y;
            }
        }

        public override int GetHashCode()
        {
            unchecked
            {
                int hash = 17;
                hash = hash * 23 + X.GetHashCode();
                hash = hash * 23 + Y.GetHashCode();
                return hash;
            }
        }
        #endregion

        #region Operators
        public static bool operator ==(BigVec2Int left, BigVec2Int right)
        {
            return left.X == right.X && left.Y == right.Y;
        }
        
        public static bool operator !=(BigVec2Int left, BigVec2Int right)
        {
            return left.X != right.X || left.Y != right.Y;
        }

        public static BigVec2Int operator +(BigVec2Int left, BigVec2Int right)
        {
            return new BigVec2Int(left.X + right.X, left.Y + right.Y);
        }

        public static BigVec2Int operator -(BigVec2Int left, BigVec2Int right)
        {
            return new BigVec2Int(left.X - right.X, left.Y - right.Y);
        }

        public static BigVec2Int operator *(BigVec2Int left, int right)
        {
            return new BigVec2Int(left.X * right, left.Y * right);
        }

        public static BigVec2Int operator /(BigVec2Int left, int right)
        {
            return new BigVec2Int(left.X / right, left.Y / right);
        }

        public static explicit operator UnityEngine.Vector2(BigVec2Int bigVec2Int)
        {
            return new UnityEngine.Vector2((float)bigVec2Int.X, (float)bigVec2Int.Y);
        }

        public static explicit operator UnityEngine.Vector3(BigVec2Int bigVec2Int)
        {
            return new UnityEngine.Vector3((float)bigVec2Int.X, (float)bigVec2Int.Y);
        }

        public static explicit operator BigVec2Int(UnityEngine.Vector2 vector2)
        {
            return new BigVec2Int((BigInteger)vector2.x, (BigInteger)vector2.y);
        }

        public static explicit operator BigVec2Int(UnityEngine.Vector3 vector3)
        {
            return new BigVec2Int((BigInteger)vector3.x, (BigInteger)vector3.y);
        }
        #endregion
    }
}
