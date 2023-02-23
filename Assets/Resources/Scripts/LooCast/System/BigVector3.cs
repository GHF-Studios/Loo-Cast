namespace LooCast.System
{
    public struct BigVector3
    {
        public BigFloat X { get; set; }
        public BigFloat Y { get; set; }
        public BigFloat Z { get; set; }

        public BigVector3(BigFloat x, BigFloat y, BigFloat z)
        {
            X = x;
            Y = y;
            Z = z;
        }
    }
}