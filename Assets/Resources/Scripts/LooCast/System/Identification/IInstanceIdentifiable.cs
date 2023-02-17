using System;

namespace LooCast.System.Identification
{
    public interface IInstanceIdentifiable : IIdentifiable
    {
        #region Properties
        IInstanceIdentifier InstanceIdentifier { get; }
        #endregion
    }
}
