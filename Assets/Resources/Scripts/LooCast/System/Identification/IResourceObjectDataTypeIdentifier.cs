using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceObjectDataTypeIdentifier : IResourceDataTypeIdentifier
    {
        #region Properties
        string ParentResourceObjectDataTypeID { get; }
        string ResourceObjectDataTypeID { get; }
        #endregion
    }
}