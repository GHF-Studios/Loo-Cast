using System.Collections.Generic;

namespace LooCast.System
{
    using System.Identification;
    
    public interface INamespace : IIdentifiable
    {
        public string NamespaceName { get; }
        public INamespace ParentNamespace { get; }
        public List<INamespace> ChildNamespaces { get; }
    }
}