using System;

namespace LooCast.System.Identifiers
{
    public interface IRegistryIdentifier : IObjectIdentifier
    {
        #region Properties
        public string RegistryGUSID { get; }
        
        ITypeIdentifier KeyTypeIdentifier { get; }
        ITypeIdentifier ValueTypeIdentifier { get; }
        #endregion
    }
}
