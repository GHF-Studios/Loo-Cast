using LooCast.Mod;
using System.Collections.Generic;

namespace LooCast.Identifier
{
    public class IdentifierManager
    {
        #region Static Properties
        public static IdentifierManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new IdentifierManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static IdentifierManager instance;
        #endregion

        #region  Properties
        public TypeIdentifier RootTypeIdentifier
        {
            get
            {
                return _rootTypeIdentifier;
            }
        }
        #endregion

        #region Fields
        private TypeIdentifier _rootTypeIdentifier;
        private Dictionary<string, TypeIdentifier> _typeIdentifiers;
        private Dictionary<string, InstanceIdentifier> _instanceIdentifiers;
        #endregion

        #region Methods
        public void RegisterTypeIdentifier(TypeIdentifier typeIdentifier)
        {
            typeIdentifier.Parent.Children.Add(typeIdentifier);
        }

        public void UnregisterTypeIdentifier(string typeIdentifierGUID)
        {
            TypeIdentifier typeIdentifier = GetTypeIdentifier(typeIdentifierGUID);
            typeIdentifier.Parent.Children.Remove(typeIdentifier);
        }

        public TypeIdentifier GetTypeIdentifier(string typeIdentifierGUID)
        {
            return _typeIdentifiers[typeIdentifierGUID];
        }

        public bool ContainsTypeIdentifier(string typeIdentifierGUID)
        {
            return _typeIdentifiers.ContainsKey(typeIdentifierGUID);
        }

        public void RegisterInstanceIdentifier(InstanceIdentifier instanceIdentifier)
        {
            _instanceIdentifiers.Add(instanceIdentifier.GUID, instanceIdentifier);
        }

        public void UnregisterInstanceIdentifier(string instanceIdentifierGUID)
        {
            _instanceIdentifiers.Remove(instanceIdentifierGUID);
        }

        public InstanceIdentifier GetInstanceIdentifier(string instanceIdentifierGUID)
        {
            return _instanceIdentifiers[instanceIdentifierGUID];
        }

        public bool ContainsInstanceIdentifier(string instanceIdentifierGUID)
        {
            return _instanceIdentifiers.ContainsKey(instanceIdentifierGUID);
        }

        internal void Initialize()
        {
            _rootTypeIdentifier = new TypeIdentifier();
            _typeIdentifiers = new Dictionary<string, TypeIdentifier>();
            _instanceIdentifiers = new Dictionary<string, InstanceIdentifier>();
        }
        #endregion
    } 
}