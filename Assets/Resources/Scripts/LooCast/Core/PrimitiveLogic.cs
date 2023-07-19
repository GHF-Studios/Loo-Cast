using System;

namespace LooCast.Core
{
    public abstract class PrimitiveLogic
    {
        #region Properties
        public Guid LogicID { get; private set; }
        #endregion

        #region Constructors
        protected PrimitiveLogic()
        {
            LogicID = new Guid();
        }
        #endregion

        #region Overrides
        public override int GetHashCode()
        {
            return LogicID.GetHashCode();
        }

        public override bool Equals(object obj)
        {
            if (obj is not PrimitiveLogic)
            {
                return false;
            }

            PrimitiveLogic other = (PrimitiveLogic)obj;
            return other.LogicID == this.LogicID;
        }

        public override string ToString()
        {
            return LogicID.ToString();
        }
        #endregion
    }
}
