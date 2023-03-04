using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceObjectMetaDataTypeIdentifier : IResourceMetaDataTypeIdentifier
    {
        #region Properties
        string ParentResourceObjectMetaDataTypeID { get; }
        string ResourceObjectMetaDataTypeID { get; }
        #endregion
    }
}