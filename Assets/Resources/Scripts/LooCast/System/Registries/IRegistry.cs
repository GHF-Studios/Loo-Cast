using System.Collections.Generic;

namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;
    
    public interface IRegistry : IEngineObject, IChild<IRegistry>, IParent<IRegistry>, IParent<IIdentifiableObject>
    {
        #region Properties
        public IRegistryIdentifier RegistryIdentifier { get; }
        IRegistry RegistryParent { get; }
        List<IRegistry> RegistryChildren { get; }
        List<IIdentifiableObject> IdentifiableObjectChildren { get; }
        #endregion

        #region Methods
        void AddObject(IObjectIdentifier objectIdentifier, IIdentifiableObject identifiableObject);
        bool RemoveObject(IObjectIdentifier objectIdentifier);
        IIdentifiableObject GetObject(IObjectIdentifier objectIdentifier);
        bool TryGetObject(IObjectIdentifier objectIdentifier, out IIdentifiableObject identifiableObject);
        bool ContainsIdentifier(IObjectIdentifier objectIdentifier);
        bool ContainsObject(IIdentifiableObject identifiableObject);
        void Clear();
        #endregion
    }

    public interface IRegistry<IdentifierType, ObjectType> : IRegistry
        where IdentifierType : IObjectIdentifier
        where ObjectType : IIdentifiableObject
    {
        #region Methods
        void AddObject(IdentifierType objectIdentifier, ObjectType identifiableObject);
        bool RemoveObject(IdentifierType objectIdentifier);
        ObjectType GetObject(IdentifierType objectIdentifier);
        bool TryGetObject(IdentifierType objectIdentifier, out ObjectType identifiableObject);
        bool ContainsIdentifier(IdentifierType objectIdentifier);
        bool ContainsObject(ObjectType identifiableObject);
        #endregion
    }
}
