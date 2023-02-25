namespace LooCast.System
{
    public struct BigVector3
    {
        #region Properties
        public BigFloat X { get; set; }
        public BigFloat Y { get; set; }
        public BigFloat Z { get; set; }
        public BigFloat Length => BigFloat.Sqrt(X * X + Y * Y + Z * Z);
        #endregion

        #region Constructors
        public BigVector3(BigFloat x, BigFloat y, BigFloat z)
        {
            X = x;
            Y = y;
            Z = z;
        }
        #endregion

        #region Methods
        public BigVector3 Normalize()
        {
            var length = Length;
            return new BigVector3(X / length, Y / length, Z / length);
        }

        public BigFloat Dot(BigVector3 other)
        {
            return X * other.X + Y * other.Y + Z * other.Z;
        }

        public BigVector3 Cross(BigVector3 other)
        {
            return new BigVector3(Y * other.Z - Z * other.Y, Z * other.X - X * other.Z, X * other.Y - Y * other.X);
        }

        public BigVector3 Multiply(BigFloat scalar) => new BigVector3(X * scalar, Y * scalar, Z * scalar);
        #endregion
    }
}