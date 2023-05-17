using UnityEngine;
using System;
using System.Collections.Generic;

namespace LooCast.System.Data
{
    [Serializable]
    public abstract class Data : IData
    {
        #region Properties
        public abstract IInstance Instance { get; }
        public abstract IIdentifier Identifier { get; }
        public abstract HierarchyElement HierarchyElement { get; }
        public abstract IData DataParent { get; }
        public abstract IEnumerable<IData> DataChildren { get; }
        #endregion

        #region Fields
        [SerializeField] private string gusid;
        [SerializeField] private string gusp;

        [SerializeField] private string gusidParent;
        [SerializeField] private string guspParent;

        [SerializeField] private string[] gusidChildren;
        [SerializeField] private string[] guspChildren;
        #endregion

        #region Constructors
        protected Data(string gusid, string gusp, string gusidParent, string guspParent, string[] gusidChildren, string[] guspChildren)
        {
            this.gusid = gusid;
            this.gusp = gusp;

            this.gusidParent = gusidParent;
            this.guspParent = guspParent;

            this.gusidChildren = gusidChildren;
            this.guspChildren = guspChildren;

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

            if (gusidChildren == null || gusidChildren.Length == 0)
            {
                return false;
            }
            if (guspChildren == null || guspChildren.Length == 0)
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
