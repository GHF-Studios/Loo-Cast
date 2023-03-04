using System;

namespace LooCast.System.Identification
{
    public interface IResourceMetaDataIdentifiable : IObjectMetaDataIdentifiable
    {
        #region Properties
        IResourceMetaDataIdentifiable ResourceMetaDataIdentifier { get; }
        #endregion
    }
}
