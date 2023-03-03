using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceObjectTypeIdentifier : IResourceTypeIdentifier
    {
        #region Properties
        string ParentResourceObjectTypeID { get; }
        string ResourceObjectTypeID { get; }
        #endregion
    }
}