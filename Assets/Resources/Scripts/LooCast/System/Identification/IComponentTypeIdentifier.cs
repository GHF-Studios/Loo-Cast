using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface IComponentTypeIdentifier : IGameObjectTypeIdentifier
    {
        #region Properties
        string ParentComponentTypeID { get; }
        string ComponentTypeID { get; }
        #endregion
    }
}