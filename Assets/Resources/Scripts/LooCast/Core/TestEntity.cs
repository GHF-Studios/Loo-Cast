namespace LooCast.Core
{
    public class TestEntity : Entity
    {
        #region Constructors
        public TestEntity() : base("TestEntity")
        {
        }
        #endregion

        #region Overrides
        public override void EnableUnityBridge()
        {
            base.EnableUnityBridge();
            UnityBridge.RootGameObject.name = "TestEntity";
        }
        #endregion
    }
}
