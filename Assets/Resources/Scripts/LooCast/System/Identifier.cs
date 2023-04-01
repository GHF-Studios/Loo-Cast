namespace LooCast.System
{
    public class Identifier
    {
        #region Properties
        public string GUSID => gusid;
        public HierarchyPath HierarchyPath => hierarchyPath;
        #endregion

        #region Fields
        private string gusid;
        private HierarchyPath hierarchyPath;
        #endregion

        #region Constructors
        public Identifier(string gusid)
        {
            this.gusid = gusid;
            hierarchyPath = 
        }
        #endregion
    }
}
