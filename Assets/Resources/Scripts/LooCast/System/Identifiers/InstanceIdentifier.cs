using System;

namespace LooCast.System.Identifiers
{
    [Serializable]
    public abstract class InstanceIdentifier : Identifier, IInstanceIdentifier
    {
        #region Constructors
        protected InstanceIdentifier(string gusid) : base(gusid)
        {
        }
        #endregion
    }
}
