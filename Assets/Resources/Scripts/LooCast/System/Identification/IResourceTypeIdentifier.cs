using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceTypeIdentifier : IObjectTypeIdentifier
    {
        #region Properties
        string ParentResourceTypeID { get; }
        string ResourceTypeID { get; }
        #endregion
    }
}