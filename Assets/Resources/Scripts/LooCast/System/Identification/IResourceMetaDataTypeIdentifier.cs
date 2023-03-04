using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceMetaDataTypeIdentifier : IObjectMetaDataTypeIdentifier
    {
        #region Properties
        string ParentResourceMetaDataTypeID { get; }
        string ResourceMetaDataTypeID { get; }
        #endregion
    }
}