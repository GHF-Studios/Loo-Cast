using UnityEngine;
using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    [Serializable]
    public abstract class Data : IData
    {
        #region Properties
        public abstract IMetaData ContainingMetaData { get; }
        
        public abstract IData DataParent { get; }
        #endregion

        #region Fields
        [SerializeField] protected string gusid;
        [SerializeField] protected string gusp;

        [SerializeField] protected string gusidParent;
        [SerializeField] protected string guspParent;
        #endregion

        #region Constructors
        protected Data(string gusid, string gusp, string gusidParent, string guspParent)
        {
            this.gusid = gusid;
            this.gusp = gusp;

            this.gusidParent = gusidParent;
            this.guspParent = guspParent;

            if (!Validate())
            {
                throw new Exception("Data is not valid!");
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
