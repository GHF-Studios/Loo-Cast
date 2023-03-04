using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IInstanceMetaDataTypeIdentifier : IMetaDataTypeIdentifier
    {
        #region Properties
        string ParentInstanceMetaDataTypeID { get; }
        string InstanceMetaDataTypeID { get; }
        #endregion
    }
}