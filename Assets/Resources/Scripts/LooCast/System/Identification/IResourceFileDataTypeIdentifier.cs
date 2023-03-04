using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceFileDataTypeIdentifier : IResourceObjectDataTypeIdentifier
    {
        #region Properties
        string ParentResourceFileDataTypeID { get; }
        string ResourceFileDataTypeID { get; }
        #endregion
    }
}