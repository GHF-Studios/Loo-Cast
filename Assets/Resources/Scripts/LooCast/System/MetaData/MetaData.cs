using UnityEngine;
using System;
using System.Collections.Generic;
using LooCast.System.Identifiers;

namespace LooCast.System.MetaData
{
    [Serializable]
    public abstract class MetaData : IMetaData
    {
        #region Properties
        public IIdentifier ObjectIdentifier => MetaDataIdentifier;
        public abstract IMetaDataIdentifier MetaDataIdentifier { get; }

        public HierarchyElement ObjectHierarchyElement { get; }

        public abstract IMetaData MetaDataParent { get; }
        #endregion

        #region Fields
        [SerializeField] protected string gusid;
        [SerializeField] protected string gusp;

        [SerializeField] protected string gusidParent;
        [SerializeField] protected string guspParent;
        #endregion

        #region Constructors
        protected MetaData(string gusid, string gusp, string gusidParent, string guspParent)
        {
            this.gusid = gusid;
            this.gusp = gusp;
            
            this.gusidParent = gusidParent;
            this.guspParent = guspParent;

            if (!Validate())
            {
                throw new Exception("MetaData is not valid!");
            }
        }
        #endregion

        #region Methods
        public virtual bool Validate()
        {
            if (string.IsNullOrEmpty(gusid) || string.IsNullOrWhiteSpace(gusid))
            {
                return false;
            }
            if (string.IsNullOrEmpty(gusp) || string.IsNullOrWhiteSpace(gusp))
            {
                return false;
            }

            if (string.IsNullOrEmpty(gusidParent) || string.IsNullOrWhiteSpace(gusidParent))
            {
                return false;
            }
            if (string.IsNullOrEmpty(guspParent) || string.IsNullOrWhiteSpace(guspParent))
            {
                return false;
            }

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
