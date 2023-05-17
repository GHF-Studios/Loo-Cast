using System;

namespace LooCast.System.Identifiers
{
    public interface IInstanceIdentifier : IIdentifier
    {
        #region Properties
        public string InstanceGUSID => GUSID;
        #endregion
    }
}
