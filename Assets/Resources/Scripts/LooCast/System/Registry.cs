﻿using System.Collections.Generic;

namespace LooCast.System
{
    using LooCast.System.Identifiers;
    using LooCast.System.Managers;

    public abstract class Registry<KeyType, ValueType> : IIdentifiable, IDictionary<KeyType, ValueType> where KeyType : Identifier where ValueType : IIdentifiable
    {
        #region Properties
        public Identifier Identifier => systemObject.Identifier;
        
        public ValueType this[KeyType key] 
        { 
            get => ((IDictionary<KeyType, ValueType>)dictionary)[key]; 
            set => ((IDictionary<KeyType, ValueType>)dictionary)[key] = value; 
        }
        public ICollection<KeyType> Keys => ((IDictionary<KeyType, ValueType>)dictionary).Keys;
        public ICollection<ValueType> Values => ((IDictionary<KeyType, ValueType>)dictionary).Values;
        public int Count => ((ICollection<KeyValuePair<KeyType, ValueType>>)dictionary).Count;
        public bool IsReadOnly => ((ICollection<KeyValuePair<KeyType, ValueType>>)dictionary).IsReadOnly;
        #endregion

        #region Fields
        private SystemObject systemObject;
        
        private Dictionary<KeyType, ValueType> dictionary;
        #endregion

        #region Constructors
        public Registry()
        {
            dictionary = new Dictionary<KeyType, ValueType>();
            
            if (!TypeIdentifier.TryParse("LooCast.System:Registry<LooCast.System:Identifier, LooCast.System:IIdentifiable>", out TypeIdentifier? systemObjectContainingTypeIdentifier))
            {
                throw new global::System.Exception("Could not parse TypeIdentifier from string.");
            }
            
            TypeManager typeManager = TypeManager.Instance;
            systemObject = new SystemObject(new global::System.Guid(), this, TypeManager.Instance.GetType(systemObjectContainingTypeIdentifier));
        }
        #endregion

        #region Methods
        public void Add(KeyType key, ValueType value)
        {
            ((IDictionary<KeyType, ValueType>)dictionary).Add(key, value);
        }

        public void Add(KeyValuePair<KeyType, ValueType> item)
        {
            ((ICollection<KeyValuePair<KeyType, ValueType>>)dictionary).Add(item);
        }

        public void Clear()
        {
            ((ICollection<KeyValuePair<KeyType, ValueType>>)dictionary).Clear();
        }

        public bool Contains(KeyValuePair<KeyType, ValueType> item)
        {
            return ((ICollection<KeyValuePair<KeyType, ValueType>>)dictionary).Contains(item);
        }

        public bool ContainsKey(KeyType key)
        {
            return ((IDictionary<KeyType, ValueType>)dictionary).ContainsKey(key);
        }

        public void CopyTo(KeyValuePair<KeyType, ValueType>[] array, int arrayIndex)
        {
            ((ICollection<KeyValuePair<KeyType, ValueType>>)dictionary).CopyTo(array, arrayIndex);
        }

        public IEnumerator<KeyValuePair<KeyType, ValueType>> GetEnumerator()
        {
            return ((IEnumerable<KeyValuePair<KeyType, ValueType>>)dictionary).GetEnumerator();
        }

        public bool Remove(KeyType key)
        {
            return ((IDictionary<KeyType, ValueType>)dictionary).Remove(key);
        }

        public bool Remove(KeyValuePair<KeyType, ValueType> item)
        {
            return ((ICollection<KeyValuePair<KeyType, ValueType>>)dictionary).Remove(item);
        }

        public bool TryGetValue(KeyType key, out ValueType value)
        {
            return ((IDictionary<KeyType, ValueType>)dictionary).TryGetValue(key, out value);
        }

        global::System.Collections.IEnumerator global::System.Collections.IEnumerable.GetEnumerator()
        {
            return ((global::System.Collections.IEnumerable)dictionary).GetEnumerator();
        }
        #endregion
    }
}
