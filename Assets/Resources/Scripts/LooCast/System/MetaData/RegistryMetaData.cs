using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    using LooCast.System.Registries;
    
    public class RegistryMetaData<KeyType, ValueType> : IRegistryMetaData
        where KeyType : IIdentifier
        where ValueType : IInstance
    {
        #region Properties
        public IMetaData MetaDataParent
        {
            get
            {
                return registryMetaDataParent;
            }

            set
            {
                registryMetaDataParent = (IRegistryMetaData)value;
            }
        }
        public IRegistryMetaData RegistryMetaDataParent
        {
            get
            {
                return registryMetaDataParent;
            }

            set
            {
                registryMetaDataParent = value;
            }
        }

        public IEnumerable<IMetaData> MetaDataChildren
        {
            get
            {
                return registryMetaDataChildren;
            }

            set
            {
                registryMetaDataChildren = (IEnumerable<IRegistryMetaData>)value;
            }
        }
        public IEnumerable<IRegistryMetaData> RegistryMetaDataChildren
        {
            get
            {
                return registryMetaDataChildren;
            }

            set
            {
                registryMetaDataChildren = value;
            }
        }

        public ILooCastObject Parent
        {
            get
            {
                return registryParent;
            }

            set
            {
                registryParent = (IRegistry)value;
            }
        }
        public IRegistry RegistryParent
        {
            get
            {
                return registryParent;
            }

            set
            {
                registryParent = value;
            }
        }

        public IEnumerable<ILooCastObject> Children
        {
            get
            {
                return registryChildren;
            }

            set
            {
                registryChildren = (IEnumerable<IRegistry>)value;
            }
        }
        public IEnumerable<IRegistry> RegistryChildren
        {
            get
            {
                return registryChildren;
            }

            set
            {
                registryChildren = value;
            }
        }
        #endregion

        #region Fields
        #endregion

        #region Constructors
        
        #endregion
    }
}
