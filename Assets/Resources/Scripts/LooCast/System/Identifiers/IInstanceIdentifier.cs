using System;

namespace LooCast.System.Identifiers
{
    public interface IInstanceIdentifier : IObjectIdentifier
    {
        #region Properties
        public string InstanceGUSID => GUSID;
        #endregion
    }
}
