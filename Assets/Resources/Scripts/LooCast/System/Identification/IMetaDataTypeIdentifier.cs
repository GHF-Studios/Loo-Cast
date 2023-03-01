using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IMetaDataTypeIdentifier : IObjectTypeIdentifier
    {
        #region Properties
        string ParentMetaDataTypeID { get; }
        string MetaDataTypeID { get; }
        #endregion
    }
}