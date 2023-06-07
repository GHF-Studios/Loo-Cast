using System.Collections.Generic;

namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;
    
    public interface IRegistry : IEngineObject, IHierarchyElement, IChild<IRegistry>, IParent<IRegistry>, IParent<IIdentifiableObject>
    {
        #region Properties
        public IRegistryIdentifier RegistryIdentifier { get; }
        IRegistry RegistryParent { get; }
        List<IRegistry> RegistryChildren { get; }
        List<IEngineObject> IdentifiableObjectChildren { get; }
        #endregion

        #region Methods
        public void Add(IObjectIdentifier key, IIdentifiableObject value);
        public bool Remove(IObjectIdentifier key);
        public IIdentifiableObject Get(IObjectIdentifier key);
        public bool ContainsKey(IObjectIdentifier key);
        public bool ContainsValue(IIdentifiableObject value);
        public void Clear();
        #endregion
    }
}
