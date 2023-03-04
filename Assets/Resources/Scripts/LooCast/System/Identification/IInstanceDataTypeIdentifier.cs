using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IInstanceDataTypeIdentifier : IDataTypeIdentifier
    {
        #region Properties
        string ParentInstanceDataTypeID { get; }
        string InstanceDataTypeID { get; }
        #endregion
    }
}