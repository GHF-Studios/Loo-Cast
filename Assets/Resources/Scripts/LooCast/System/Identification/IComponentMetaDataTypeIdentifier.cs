using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IComponentMetaDataTypeIdentifier : IGameObjectMetaDataTypeIdentifier
    {
        #region Properties
        string ParentComponentMetaDataTypeID { get; }
        string ComponentMetaDataTypeID { get; }
        #endregion
    }
}