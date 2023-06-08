using System.Collections.Generic;

namespace LooCast.System.Registries
{
    using LooCast.System.Identifiers;
    using LooCast.System.Paths;

    public class Registry<IdentifierType, ObjectType> : IRegistry
        where IdentifierType : IObjectIdentifier
        where ObjectType : IIdentifiableObject
    {
        #region Properties
        public IObjectIdentifier ObjectIdentifier => RegistryIdentifier;
        public IRegistryIdentifier RegistryIdentifier { get; private set; }

        public IHierarchicalElementPath HierarchicalElementPath => RegistryFolderPath;
        public FolderPath RegistryFolderPath { get; private set; }

        public HierarchicalElementType HierarchyElementType => HierarchicalElementType.Folder;

        public IEngineObject Parent => ((IChild<IRegistry>)this).Parent;
        IRegistry IChild<IRegistry>.Parent => RegistryParent;
        public IRegistry RegistryParent { get; private set; }

        public IEnumerable<IEngineObject> Children => ((IParent<IRegistry>)this).Children;
        IEnumerable<IRegistry> IParent<IRegistry>.Children => RegistryChildren;
        public List<IRegistry> RegistryChildren { get; private set; }

        IEnumerable<IIdentifiableObject> IParent<IIdentifiableObject>.Children => IdentifiableObjectChildren;
        public List<IIdentifiableObject> IdentifiableObjectChildren { get; private set; }

        #region Initialization Phase Flags
        public bool IsEarlyPreInitializing { get; private set; }
        public bool IsPreInitializing { get; private set; }
        public bool IsLatePreInitializing { get; private set; }
        public bool IsEarlyPreInitialized { get; private set; }
        public bool IsPreInitialized { get; private set; }
        public bool IsLatePreInitialized { get; private set; }

        public bool IsEarlyInitializing { get; private set; }
        public bool IsInitializing { get; private set; }
        public bool IsLateInitializing { get; private set; }
        public bool IsEarlyInitialized { get; private set; }
        public bool IsInitialized { get; private set; }
        public bool IsLateInitialized { get; private set; }

        public bool IsEarlyPostInitializing { get; private set; }
        public bool IsPostInitializing { get; private set; }
        public bool IsLatePostInitializing { get; private set; }
        public bool IsEarlyPostInitialized { get; private set; }
        public bool IsPostInitialized { get; private set; }
        public bool IsLatePostInitialized { get; private set; }

        public bool IsFullyPreInitialized
        {
            get
            {
                return IsEarlyPreInitialized && IsPreInitialized && IsLatePreInitialized;
            }
        }
        public bool IsFullyInitialized
        {
            get
            {
                return IsEarlyInitialized && IsInitialized && IsLateInitialized;
            }
        }
        public bool IsFullyPostInitialized
        {
            get
            {
                return IsEarlyPostInitialized && IsPostInitialized && IsLatePostInitialized;
            }
        }
        public bool IsCompletelyInitialized
        {
            get
            {
                return IsFullyPreInitialized && IsFullyInitialized && IsPostInitialized;
            }
        }
        #endregion

        #region Termination Phase Flags
        public bool IsEarlyPreTerminating { get; private set; }
        public bool IsPreTerminating { get; private set; }
        public bool IsLatePreTerminating { get; private set; }
        public bool IsEarlyPreTerminated { get; private set; }
        public bool IsPreTerminated { get; private set; }
        public bool IsLatePreTerminated { get; private set; }

        public bool IsEarlyTerminating { get; private set; }
        public bool IsTerminating { get; private set; }
        public bool IsLateTerminating { get; private set; }
        public bool IsEarlyTerminated { get; private set; }
        public bool IsTerminated { get; private set; }
        public bool IsLateTerminated { get; private set; }

        public bool IsEarlyPostTerminating { get; private set; }
        public bool IsPostTerminating { get; private set; }
        public bool IsLatePostTerminating { get; private set; }
        public bool IsEarlyPostTerminated { get; private set; }
        public bool IsPostTerminated { get; private set; }
        public bool IsLatePostTerminated { get; private set; }

        public bool IsFullyPreTerminated
        {
            get
            {
                return IsEarlyPreTerminated && IsPreTerminated && IsLatePreTerminated;
            }
        }
        public bool IsFullyTerminated
        {
            get
            {
                return IsEarlyTerminated && IsTerminated && IsLateTerminated;
            }
        }
        public bool IsFullyPostTerminated
        {
            get
            {
                return IsEarlyPostTerminated && IsPostTerminated && IsLatePostTerminated;
            }
        }
        public bool IsCompletelyTerminated
        {
            get
            {
                return IsFullyPreTerminated && IsFullyTerminated && IsPostTerminated;
            }
        }
        #endregion

        #endregion

        #region Fields
        private Dictionary<IdentifierType, ObjectType> dictionary;
        #endregion

        #region Constructors
        public Registry(IRegistry registryParent)
        {
            RegistryIdentifier = Identifiers.RegistryIdentifier.Parse<IdentifierType, ObjectType>();
            //RegistryFolderPath = ;

            RegistryParent = registryParent;
            RegistryChildren = new List<IRegistry>();
            IdentifiableObjectChildren = new List<IIdentifiableObject>();

            dictionary = new Dictionary<IdentifierType, ObjectType>();
        }
        #endregion

        #region Methods
        public virtual bool Validate()
        {
            return true;
        }
        
        public void AddObject(IObjectIdentifier objectIdentifier, IIdentifiableObject identifiableObject)
        {
            if (!(objectIdentifier is IdentifierType))
            {
                throw new global::System.Exception($"Identifier type {objectIdentifier.GetType()} is not of type {typeof(IdentifierType)}");
            }
            if (!(identifiableObject is ObjectType))
            {
                throw new global::System.Exception($"Object type {identifiableObject.GetType()} is not of type {typeof(ObjectType)}");
            }
            
            AddObject((IdentifierType)objectIdentifier, (ObjectType)identifiableObject);
        }
        public void AddObject(IdentifierType objectIdentifier, ObjectType identifiableObject)
        {
            dictionary.Add(objectIdentifier, identifiableObject);
            RegistryParent?.AddObject(objectIdentifier, identifiableObject);
        }

        public bool RemoveObject(IObjectIdentifier objectIdentifier)
        {
            if (!(objectIdentifier is IdentifierType))
            {
                throw new global::System.Exception($"Identifier type {objectIdentifier.GetType()} is not of type {typeof(IdentifierType)}");
            }
            
            return RemoveObject((IdentifierType)objectIdentifier);
        }
        public bool RemoveObject(IdentifierType objectIdentifier)
        {
            bool removed = dictionary.Remove(objectIdentifier);
            if (RegistryParent != null)
            {
                removed &= RegistryParent.RemoveObject(objectIdentifier);
            }
            return removed;
        }

        public IIdentifiableObject GetObject(IObjectIdentifier objectIdentifier)
        {
            if (!(objectIdentifier is IdentifierType))
            {
                throw new global::System.Exception($"Identifier type '{objectIdentifier.GetType()}' is not of type '{typeof(IdentifierType)}'!");
            }
            
            return GetObject((IdentifierType)objectIdentifier);
        }
        public ObjectType GetObject(IdentifierType objectIdentifier)
        {
            if (TryGetObject(objectIdentifier, out ObjectType value))
            {
                return value;
            }
            throw new global::System.Exception($"Object of type '{typeof(ObjectType)}' with identifier '{objectIdentifier}' not found!");
        }

        public bool TryGetObject(IObjectIdentifier objectIdentifier, out IIdentifiableObject identifiableObject)
        {
            if (!(objectIdentifier is IdentifierType))
            {
                throw new global::System.Exception($"Identifier type '{objectIdentifier.GetType()}' is not of type '{typeof(IdentifierType)}'!");
            }

            return TryGetObject((IdentifierType)objectIdentifier, out identifiableObject);
        }
        public bool TryGetObject(IdentifierType objectIdentifier, out ObjectType identifiableObject)
        {
            return dictionary.TryGetValue(objectIdentifier, out identifiableObject);
        }

        public bool ContainsIdentifier(IObjectIdentifier objectIdentifier)
        {
            if (!(objectIdentifier is IdentifierType))
            {
                throw new global::System.Exception($"Identifier type '{objectIdentifier.GetType()}' is not of type '{typeof(IdentifierType)}'!");
            }

            return ContainsIdentifier((IdentifierType)objectIdentifier);
        }
        public bool ContainsIdentifier(IdentifierType objectIdentifier)
        {
            return dictionary.ContainsKey(objectIdentifier);
        }

        public bool ContainsObject(IIdentifiableObject identifiableObject)
        {
            if (!(identifiableObject is ObjectType))
            {
                throw new global::System.Exception($"Object type '{identifiableObject.GetType()}' is not of type '{typeof(ObjectType)}'!");
            }

            return ContainsObject((ObjectType)identifiableObject);
        }
        public bool ContainsObject(ObjectType identifiableObject)
        {
            return dictionary.ContainsValue(identifiableObject);
        }

        public void Clear()
        {
            dictionary.Clear();
            RegistryParent?.Clear();
        }

        #region Initialization Phases
        public virtual void EarlyPreInitialize()
        {
            
        }

        public virtual void PreInitialize()
        {
            
        }

        public virtual void LatePreInitialize()
        {
            
        }

        public virtual void EarlyInitialize()
        {
            
        }

        public virtual void Initialize()
        {
            
        }

        public virtual void LateInitialize()
        {
            
        }

        public virtual void EarlyPostInitalize()
        {
            
        }

        public virtual void PostInitialize()
        {
            
        }

        public virtual void LatePostInitialize()
        {
            
        }
        #endregion

        #region Termination Phases
        public virtual void EarlyPreTerminate()
        {
            
        }

        public virtual void PreTerminate()
        {
            
        }

        public virtual void LatePreTerminate()
        {
            
        }

        public virtual void EarlyTerminate()
        {
            
        }

        public virtual void Terminate()
        {
            
        }

        public virtual void LateTerminate()
        {
            
        }

        public virtual void EarlyPostTerminate()
        {
            
        }

        public virtual void PostTerminate()
        {
            
        }

        public virtual void LatePostTerminate()
        {
            
        }
        #endregion

        #endregion
    }
}
