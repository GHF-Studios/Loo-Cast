using CSSystem = System;

namespace LooCast.System.Identification
{
    public interface ITypeIdentifier : IGenericIdentifier<Type>
    {
        #region Properties
        string ParentNamespaceID { get; }
        string ParentTypeID { get; }
        CSSystem.Type CSSystemType { get; }
        string TypeID { get; }
        #endregion
    }
}