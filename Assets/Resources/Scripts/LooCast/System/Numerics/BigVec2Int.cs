using System.Numerics;
using UnityEngine;

namespace LooCast.System.Numerics
{
    public struct BigVec2Int
    {
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

        public static BigVec2Int operator *(BigVec2Int left, int right)
        {
            return new BigVec2Int(left.X * right, left.Y * right);
        }

        public static explicit operator UnityEngine.Vector2(BigVec2Int bigVec2Int)
        {
            return new UnityEngine.Vector2((float)bigVec2Int.X, (float)bigVec2Int.Y);
        }

        public static explicit operator UnityEngine.Vector3(BigVec2Int bigVec2Int)
        {
            return new UnityEngine.Vector3((float)bigVec2Int.X, (float)bigVec2Int.Y);
        }
        #endregion
    }
}
