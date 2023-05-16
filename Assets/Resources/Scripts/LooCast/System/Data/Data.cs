using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.System.Data
{
    public abstract class Data : IData
    {
        #region Properties
        public abstract IIdentifier Identifier { get; }
        public abstract HierarchyElement HierarchyElement { get; }
        
        public abstract IData DataParent { get; set; }
        
        public abstract IEnumerable<IData> DataChildren { get; set; }
        #endregion

        #region Fields
        [SerializeField] private string dataGUSID;
        [SerializeField] private string dataGUSP;

        [SerializeField] private string dataGUSIDParent;
        [SerializeField] private string dataGUSPParent;

        [SerializeField] private string[] dataGUSIDChildren;
        [SerializeField] private string[] dataGUSPChildren;
        #endregion

        #region Constructors
        protected Data(string dataGUSID, string dataGUSP, string dataGUSIDParent, string dataGUSPParent, string[] dataGUSIDChildren, string[] dataGUSPChildren)
        {
            this.dataGUSID = dataGUSID;
            this.dataGUSP = dataGUSP;
            
            this.dataGUSIDParent = dataGUSIDParent;
            this.dataGUSPParent = dataGUSPParent;
            
            this.dataGUSIDChildren = dataGUSIDChildren;
            this.dataGUSPChildren = dataGUSPChildren;
        }
        #endregion

        #region Methods
        public abstract bool Validate()
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
