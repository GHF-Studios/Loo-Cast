using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IComponentDataTypeIdentifier : IGameObjectDataTypeIdentifier
    {
        #region Properties
        string ParentComponentDataTypeID { get; }
        string ComponentDataTypeID { get; }
        #endregion
    }
}