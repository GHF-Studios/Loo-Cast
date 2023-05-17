﻿using System.Collections;
using System.Collections.Generic;
using System.Linq;

namespace LooCast.System.Registries
{
    using LooCast.System.Data;
    using LooCast.System.Identifiers;
    using LooCast.System.MetaData;

    public class Registry<KeyType, ValueType> : IRegistry, IEnumerable<KeyValuePair<KeyType, ValueType>>
        where KeyType : IIdentifier
        where ValueType : IIdentifiable
    {
        #region Properties
        public IIdentifier Identifier => ObjectIdentifier;
        public IIdentifier ObjectIdentifier => RegistryIdentifier;
        public IRegistryIdentifier RegistryIdentifier => RegistryMetaData.RegistryIdentifier;

        public HierarchyElement ObjectHierarchyElement => RegistryHierarchyElement;
        public HierarchyElement RegistryHierarchyElement => test;

        public IMetaData MetaData
        {
            get
            {
                return RegistryMetaData;
            }

            set
            {
                RegistryMetaData = (IRegistryMetaData)value;
            }
        }
        public IRegistryMetaData RegistryMetaData
        {
            get
            {
                return registryMetaData;
            }

            set
            {
                registryMetaData = (RegistryMetaData<KeyType, ValueType>)value;
            }
        }

        public IData Data
        {
            get
            {
                return RegistryData;
            }

            set
            {
                RegistryData = (IRegistryData)value;
            }
        }
        public IRegistryData RegistryData
        {
            get
            {
                return registryData;
            }

            set
            {
                registryData = (RegistryData<KeyType, ValueType>)value;
            }
        }

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
        private RegistryMetaData<KeyType, ValueType> registryMetaData;
        private RegistryData<KeyType, ValueType> registryData;
        #endregion

        #region Methods
        public void Add(IIdentifier key, IIdentifiable value)
        {
            if (!(key is KeyType))
            {
                throw new global::System.Exception($"Key type {key.GetType()} is not of type {typeof(KeyType)}");
            }
            if (!(value is ValueType))
            {
                throw new global::System.Exception($"Value type {value.GetType()} is not of type {typeof(ValueType)}");
            }
            
            Add((KeyType)key, (ValueType)value);
        }

        public bool Remove(IIdentifier key)
        {
            if (!(key is KeyType))
            {
                throw new global::System.Exception($"Key type {key.GetType()} is not of type {typeof(KeyType)}");
            }
            
            return Remove((KeyType)key);
        }

        public IIdentifiable Get(IIdentifier key)
        {
            if (!(key is KeyType))
            {
                throw new global::System.Exception($"Key type '{key.GetType()}' is not of type '{typeof(KeyType)}'!");
            }
            
            return GetValue((KeyType)key);
        }

        public bool ContainsKey(IIdentifier key)
        {
            if (!(key is KeyType))
            {
                throw new global::System.Exception($"Key type '{key.GetType()}' is not of type '{typeof(KeyType)}'!");
            }

            return ContainsKey((KeyType)key);
        }

        public bool ContainsValue(IIdentifiable value)
        {
            if (!(value is ValueType))
            {
                throw new global::System.Exception($"Value type '{value.GetType()}' is not of type '{typeof(ValueType)}'!");
            }

            return dictionary.ContainsValue((ValueType)value);
        }
        
        public void Add(KeyType key, ValueType value)
        {
            dictionary.Add(key, value);
            RegistryParent?.Add(key, value);
        }

        public bool ContainsKey(KeyType key)
        {
            return dictionary.ContainsKey(key);
        }

        public bool Remove(KeyType key)
        {
            bool removed = dictionary.Remove(key);
            if (RegistryParent != null)
            {
                removed &= RegistryParent.Remove(key);
            }
            return removed;
        }

        public bool TryGetValue(KeyType key, out ValueType value)
        {
            return dictionary.TryGetValue(key, out value);
        }

        public ValueType GetValue(KeyType key)
        {
            if (TryGetValue(key, out ValueType value))
            {
                return value;
            }
            throw new global::System.Exception($"[Registry] Value of type '{typeof(ValueType)}' with key '{key}' not found!");
        }

        public IEnumerable<ValueType> GetValues(IEnumerable<KeyType> keys)
        {
            return keys.Select(key => GetValue(key));
        }
            
        public void Add(KeyValuePair<KeyType, ValueType> item)
        {
            Add(item.Key, item.Value);
            RegistryParent?.Add(item.Key, item.Value);
        }

        public void Clear()
        {
            dictionary.Clear();
            RegistryParent?.Clear();
        }

        public bool Contains(KeyValuePair<KeyType, ValueType> item)
        {
            return dictionary.Contains(item);
        }

        public bool Remove(KeyValuePair<KeyType, ValueType> item)
        {
            return Remove(item.Key);
        }

        public IEnumerator<KeyValuePair<KeyType, ValueType>> GetEnumerator()
        {
            return dictionary.GetEnumerator();
        }

        IEnumerator IEnumerable.GetEnumerator()
        {
            return dictionary.GetEnumerator();
        }

#nullable enable
        protected virtual IRegistry? GetBaseRegistry()
        {
            return null;
        }
#nullable disable

        public virtual bool Validate()
        {
            return true;
        }

        #region Initialization Phases
        public virtual void EarlyPreInitialize()
        {
            
        }

        public virtual void PreInitialize()
        {
            RegistryParent = GetBaseRegistry();
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
