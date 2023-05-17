using System;

namespace LooCast.System.Identifiers
{
    [Serializable]
    public abstract class InstanceIdentifier : Identifier, IInstanceIdentifier
    {
        #region Properties
        public string InstanceGUSID => GUSID;
        #endregion
        
        #region Constructors
        protected InstanceIdentifier(string instanceGUSID) : base(instanceGUSID)
        {
            
        }
        #endregion
    }
}
