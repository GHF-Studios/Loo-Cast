using System;

namespace LooCast.Core
{
    public abstract class PrimitiveData : IData
    {
        #region Properties
        public Guid DataID { get; private set; }
        #endregion

        #region Constructors
        protected PrimitiveData()
        {
            DataID = new Guid();
        }
        #endregion

        #region Overrides
        public override int GetHashCode()
        {
            return DataID.GetHashCode();
        }

        public override bool Equals(object obj)
        {
            if (obj is not PrimitiveData)
            {
                return false;
            }

            PrimitiveData other = (PrimitiveData)obj;
            return other.DataID == this.DataID;
        }

        public override string ToString()
        {
            return DataID.ToString();
        }
        #endregion
    }
}
