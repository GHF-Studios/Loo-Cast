using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceDataTypeIdentifier : IObjectDataTypeIdentifier
    {
        #region Properties
        string ParentResourceDataTypeID { get; }
        string ResourceDataTypeID { get; }
        #endregion
    }
}