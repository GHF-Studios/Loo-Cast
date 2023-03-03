using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IResourceFileTypeIdentifier : IResourceObjectTypeIdentifier
    {
        #region Properties
        string ParentResourceFileTypeID { get; }
        string ResourceFileTypeID { get; }
        #endregion
    }
}