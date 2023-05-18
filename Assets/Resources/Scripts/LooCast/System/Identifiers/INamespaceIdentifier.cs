using System;

namespace LooCast.System.Identifiers
{
    public interface INamespaceIdentifier : IObjectIdentifier
    {
        #region Properties
        public string NamespaceGUSID { get; }
        
        public string NamespaceName { get; }
#nullable enable
        public INamespaceIdentifier? ParentNamespaceIdentifier { get; }
#nullable disable
        #endregion
    }
}
