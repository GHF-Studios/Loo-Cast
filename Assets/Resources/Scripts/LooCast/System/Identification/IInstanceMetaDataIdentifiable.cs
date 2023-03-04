using System;

namespace LooCast.System.Identification
{
    public interface IInstanceMetaDataIdentifiable : IMetaDataIdentifiable
    {
        #region Properties
        IInstanceMetaDataIdentifiable InstanceMetaDataIdentifier { get; }
        #endregion
    }
}
