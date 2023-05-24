using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.Identifiers
{
    public class RegistryIdentifier : Identifier, IRegistryIdentifier
    {
        #region Properties
        public string RegistryGUSID => GUSID;
        
        public ITypeIdentifier KeyTypeIdentifier => keyTypeIdentifier;
        public ITypeIdentifier ValueTypeIdentifier => valueTypeIdentifier;
        #endregion

        #region Fields
        [SerializeField] private readonly ITypeIdentifier keyTypeIdentifier;
        [SerializeField] private readonly ITypeIdentifier valueTypeIdentifier;
        #endregion

        #region Constructors
#nullable enable
        protected RegistryIdentifier(ITypeIdentifier keyTypeIdentifier, ITypeIdentifier valueTypeIdentifier) : base($"{keyTypeIdentifier}_{valueTypeIdentifier}")
        {
            this.keyTypeIdentifier = keyTypeIdentifier;
            this.valueTypeIdentifier = valueTypeIdentifier;
        }
#nullable disable
        #endregion

        #region Static Methods
#nullable enable
        public static RegistryIdentifier Parse(string registryGUSID)
        {
            if (!TryParse(registryGUSID, out RegistryIdentifier? registryIdentifier))
            {
                throw new ArgumentException($"'{registryGUSID}' is not a valid registry GUSID!");
            }

            return registryIdentifier!;
        }

        public static RegistryIdentifier Parse<KeyType, ValueType>()
        where KeyType : IObjectIdentifier
        where ValueType : IIdentifiableObject
        {
            return Parse(typeof(KeyType), typeof(ValueType));
        }
        
        public static RegistryIdentifier Parse(Type keyType, Type valueType)
        {
            TypeIdentifier keyTypeIdentifier = TypeIdentifier.Parse(keyType);
            TypeIdentifier valueTypeIdentifier = TypeIdentifier.Parse(valueType);
            return Parse($"{keyTypeIdentifier}_{valueTypeIdentifier}");
        }

        public static bool TryParse(string registryGUSID, out RegistryIdentifier? registryIdentifier)
        {
            registryIdentifier = null;
            
            string[] parts = registryGUSID.Split(new char[] { '_' }, StringSplitOptions.RemoveEmptyEntries);

            if (parts.Length != 2)
            {
                return false;
            }

            if (!TypeIdentifier.TryParse(parts[0], out TypeIdentifier? keyTypeIdentifier))
            {
                return false;
            }

            if (!TypeIdentifier.TryParse(parts[1], out TypeIdentifier? valueTypeIdentifier))
            {
                return false;
            }

            registryIdentifier = new RegistryIdentifier(keyTypeIdentifier!, valueTypeIdentifier!);
            return true;
        }
#nullable disable
        #endregion

        #region Overrides
        public override string ToString()
        {
            return GUSID;
        }

        public override bool Equals(object obj)
        {
            if (obj is RegistryIdentifier)
            {
                return Equals((RegistryIdentifier)obj);
            }
            else
            {
                return false;
            }
        }

        public bool Equals(RegistryIdentifier otherRegistryIdentifier)
        {
            return otherRegistryIdentifier.GUSID.Equals(this.GUSID);
        }

        public override int GetHashCode()
        {
            return GUSID.GetHashCode();
        }
        #endregion

        #region Operators
        public static bool operator ==(RegistryIdentifier registryIdentifier1, RegistryIdentifier registryIdentifier2)
        {
            return registryIdentifier1.Equals(registryIdentifier2);
        }

        public static bool operator !=(RegistryIdentifier registryIdentifier1, RegistryIdentifier registryIdentifier2)
        {
            return !registryIdentifier1.Equals(registryIdentifier2);
        }

        public static implicit operator string(RegistryIdentifier registryIdentifier)
        {
            return registryIdentifier.GUSID;
        }

#nullable enable
        public static implicit operator RegistryIdentifier?(string registry)
        {
            if (TryParse(registry, out RegistryIdentifier? registryIdentifier))
            {
                return registryIdentifier;
            }
            else
            {
                throw new ArgumentException($"The string '{registry}' is not a valid registry.");
            }
        }
#nullable disable
        #endregion
    }
}
