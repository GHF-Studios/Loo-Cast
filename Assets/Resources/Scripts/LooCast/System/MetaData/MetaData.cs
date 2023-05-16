using UnityEngine;
using System;
using System.Collections.Generic;

namespace LooCast.System.MetaData
{
    [Serializable]
    public abstract class MetaData : IMetaData
    {
        #region Properties
        public abstract IIdentifier Identifier { get; }
        public abstract HierarchyElement HierarchyElement { get; }
        
        public abstract ILooCastObject Parent { get; set; }
        public abstract IMetaData MetaDataParent { get; set; }
        
        public abstract IEnumerable<ILooCastObject> Children { get; set; }
        public abstract IEnumerable<IMetaData> MetaDataChildren { get; set; }
        #endregion

        #region Fields
        [SerializeField] private string metaDataGUSID;
        [SerializeField] private string metaDataGUSP;
        
        [SerializeField] private string metaDataGUSIDParent;
        [SerializeField] private string metaDataGUSPParent;
        
        [SerializeField] private string[] metaDataGUSIDChildren;
        [SerializeField] private string[] metaDataGUSPChildren;
        #endregion

        #region Constructors
        protected MetaData(string metaDataGUSID, string metaDataGUSP, string metaDataGUSIDParent, string metaDataGUSPParent, string[] metaDataGUSIDChildren, string[] metaDataGUSPChildren)
        {
            this.metaDataGUSID = metaDataGUSID;
            this.metaDataGUSP = metaDataGUSP;
            
            this.metaDataGUSIDParent = metaDataGUSIDParent;
            this.metaDataGUSPParent = metaDataGUSPParent;
            
            this.metaDataGUSIDChildren = metaDataGUSIDChildren;
            this.metaDataGUSPChildren = metaDataGUSPChildren;
        }
        #endregion

        #region Methods
        public virtual bool Validate()
        {
            return true;
        }

        public virtual void PreInitialize()
        {

        }

        public virtual void Initialize()
        {

        }

        public virtual void PostInitialize()
        {

        }

        public virtual void PreTerminate()
        {

        }

        public virtual void Terminate()
        {

        }

        public virtual void PostTerminate()
        {

        }
        #endregion
    }
}
