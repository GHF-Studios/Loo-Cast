using System;

namespace LooCast.System.Identification
{
    public interface IResourceObjectMetaDataIdentifiable : IResourceMetaDataIdentifiable
    {
        #region Properties
        IResourceObjectMetaDataIdentifiable ResourceObjectMetaDataIdentifier { get; }
        #endregion
    }
}
