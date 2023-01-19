using System;
using System.Collections.Generic;

namespace LooCast.Registry
{
    using Data;

    public class SerializerRegistry : IRegistry<Serializer>
    {
        #region Properties
        public string ID
        {
            get
            {
                return "LooCast.Registry.SerializerRegistry";
            }
        }
        #endregion

        #region Fields
        private Dictionary<string, Serializer> registry;
        #endregion

        #region Constructors
        public SerializerRegistry()
        {
            registry = new Dictionary<string, Serializer>();
        }
        #endregion

        #region Methods
        public void Register(Serializer item)
        {
            registry.Add(item.ID, item);
        }

        public void Unregister(Serializer item)
        {
            registry.Remove(item.ID);
        }

        public Serializer Get(string serializerID)
        {
            return registry[serializerID];
        }

        public bool Contains(string serializerID)
        {
            return registry.ContainsKey(serializerID);
        }
        #endregion
    }
}
