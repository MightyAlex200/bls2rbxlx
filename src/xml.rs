pub const START_XML: &str = r#"
<roblox xmlns:xmime="http://www.w3.org/2005/05/xmlmime" xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance" xsi:noNamespaceSchemaLocation="http://www.roblox.com/roblox.xsd" version="4">
	<External>null</External>
	<External>nil</External>
	<Item class="Workspace" referent="RBX74CFE5D4E48245BA8EDEE8A81D9B97CB">
		<Properties>
			<bool name="AllowThirdPartySales">false</bool>
			<Ref name="CurrentCamera">RBXDB8818B284D64057B363527CE5FDF754</Ref>
			<double name="DistributedGameTime">0</double>
			<bool name="ExpSolverEnabled_Replicate">false</bool>
			<float name="FallenPartsDestroyHeight">-500</float>
			<bool name="FilteringEnabled">false</bool>
			<float name="Gravity">196.199997</float>
			<CoordinateFrame name="ModelInPrimary">
				<X>0</X>
				<Y>0</Y>
				<Z>0</Z>
				<R00>1</R00>
				<R01>0</R01>
				<R02>0</R02>
				<R10>0</R10>
				<R11>1</R11>
				<R12>0</R12>
				<R20>0</R20>
				<R21>0</R21>
				<R22>1</R22>
			</CoordinateFrame>
			<string name="Name">Workspace</string>
			<bool name="PGSPhysicsSolverEnabled">false</bool>
			<token name="PhysicalPropertiesMode">1</token>
			<Ref name="PrimaryPart">null</Ref>
			<bool name="StreamingEnabled">false</bool>
		</Properties>
		<Item class="Camera" referent="RBXDB8818B284D64057B363527CE5FDF754">
			<Properties>
				<CoordinateFrame name="CFrame">
					<X>0.640616417</X>
					<Y>17.0258446</Y>
					<Z>27.0183811</Z>
					<R00>0.444609851</R00>
					<R01>-0.606306374</R01>
					<R02>0.659328997</R02>
					<R10>2.98023224e-08</R10>
					<R11>0.736084759</R11>
					<R12>0.67688942</R12>
					<R20>-0.895724416</R20>
					<R21>-0.30095166</R21>
					<R22>0.327270508</R22>
				</CoordinateFrame>
				<Ref name="CameraSubject">null</Ref>
				<token name="CameraType">0</token>
				<float name="FieldOfView">70</float>
				<CoordinateFrame name="Focus">
					<X>-0.678041577</X>
					<Y>15.6720657</Y>
					<Z>26.3638401</Z>
					<R00>1</R00>
					<R01>0</R01>
					<R02>0</R02>
					<R10>0</R10>
					<R11>1</R11>
					<R12>0</R12>
					<R20>0</R20>
					<R21>0</R21>
					<R22>1</R22>
				</CoordinateFrame>
				<bool name="HeadLocked">true</bool>
				<string name="Name">Camera</string>
			</Properties>
		</Item>
		<Item class="Terrain" referent="RBX380E87723FE24407AC7F4BB0B4FD64A6">
			<Properties>
				<bool name="Anchored">true</bool>
				<float name="BackParamA">-0.5</float>
				<float name="BackParamB">0.5</float>
				<token name="BackSurface">0</token>
				<token name="BackSurfaceInput">0</token>
				<float name="BottomParamA">-0.5</float>
				<float name="BottomParamB">0.5</float>
				<token name="BottomSurface">4</token>
				<token name="BottomSurfaceInput">0</token>
				<int name="BrickColor">194</int>
				<CoordinateFrame name="CFrame">
					<X>0</X>
					<Y>0</Y>
					<Z>0</Z>
					<R00>1</R00>
					<R01>0</R01>
					<R02>0</R02>
					<R10>0</R10>
					<R11>1</R11>
					<R12>0</R12>
					<R20>0</R20>
					<R21>0</R21>
					<R22>1</R22>
				</CoordinateFrame>
				<bool name="CanCollide">true</bool>
				<BinaryString name="ClusterGridV3"></BinaryString>
				<Color3uint8 name="Color3uint8">4289967027</Color3uint8>
				<PhysicalProperties name="CustomPhysicalProperties">
					<CustomPhysics>false</CustomPhysics>
				</PhysicalProperties>
				<float name="Elasticity">0.300000012</float>
				<float name="Friction">0.5</float>
				<float name="FrontParamA">-0.5</float>
				<float name="FrontParamB">0.5</float>
				<token name="FrontSurface">0</token>
				<token name="FrontSurfaceInput">0</token>
				<float name="LeftParamA">-0.5</float>
				<float name="LeftParamB">0.5</float>
				<token name="LeftSurface">0</token>
				<token name="LeftSurfaceInput">0</token>
				<bool name="Locked">true</bool>
				<token name="Material">256</token>
				<string name="Name">Terrain</string>
				<float name="Reflectance">0</float>
				<float name="RightParamA">-0.5</float>
				<float name="RightParamB">0.5</float>
				<token name="RightSurface">0</token>
				<token name="RightSurfaceInput">0</token>
				<Vector3 name="RotVelocity">
					<X>0</X>
					<Y>0</Y>
					<Z>0</Z>
				</Vector3>
				<BinaryString name="SmoothGrid">AQU=</BinaryString>
				<float name="TopParamA">-0.5</float>
				<float name="TopParamB">0.5</float>
				<token name="TopSurface">3</token>
				<token name="TopSurfaceInput">0</token>
				<float name="Transparency">0</float>
				<Vector3 name="Velocity">
					<X>0</X>
					<Y>0</Y>
					<Z>0</Z>
				</Vector3>
				<Color3 name="WaterColor">4278998108</Color3>
				<float name="WaterTransparency">0.300000012</float>
				<float name="WaterWaveSize">0.150000006</float>
				<float name="WaterWaveSpeed">10</float>
				<Vector3 name="size">
					<X>2044</X>
					<Y>252</Y>
					<Z>2044</Z>
				</Vector3>
			</Properties>
		</Item>
"#;

pub const END_XML: &str = r#"
	</Item>
	<Item class="NonReplicatedCSGDictionaryService" referent="RBXA8C62DD674C342C0B2266D3921CBFF74">
		<Properties>
			<string name="Name">NonReplicatedCSGDictionaryService</string>
		</Properties>
	</Item>
	<Item class="CSGDictionaryService" referent="RBX562024AFD82B49409BA81F23EBFDC58C">
		<Properties>
			<string name="Name">CSGDictionaryService</string>
		</Properties>
	</Item>
	<Item class="Players" referent="RBX9D4E3890E16E40FA99164BBA54BE2E45">
		<Properties>
			<bool name="AllowLegacyScriptBehavior">false</bool>
			<bool name="CharacterAutoLoads">true</bool>
			<int name="MaxPlayersInternal">12</int>
			<string name="Name">Players</string>
			<int name="PreferredPlayersInternal">129731280</int>
		</Properties>
	</Item>
	<Item class="ReplicatedFirst" referent="RBX1C5A986C55B04A97AA4E68D2F0861FE8">
		<Properties>
			<string name="Name">ReplicatedFirst</string>
		</Properties>
	</Item>
	<Item class="StarterPlayer" referent="RBX17EDA5B24597463BB0CCBA9371F49B71">
		<Properties>
			<bool name="AutoJumpEnabled">true</bool>
			<float name="CameraMaxZoomDistance">400</float>
			<float name="CameraMinZoomDistance">0.5</float>
			<token name="CameraMode">0</token>
			<token name="DevCameraOcclusionMode">0</token>
			<token name="DevComputerCameraMovementMode">0</token>
			<token name="DevComputerMovementMode">0</token>
			<token name="DevTouchCameraMovementMode">0</token>
			<token name="DevTouchMovementMode">0</token>
			<bool name="EnableMouseLockOption">true</bool>
			<float name="HealthDisplayDistance">100</float>
			<bool name="LoadCharacterAppearance">true</bool>
			<string name="Name">StarterPlayer</string>
			<float name="NameDisplayDistance">100</float>
		</Properties>
		<Item class="StarterPlayerScripts" referent="RBXA8E3B35905A544E7A45A2A10674FEB8E">
			<Properties>
				<string name="Name">StarterPlayerScripts</string>
			</Properties>
		</Item>
	</Item>
	<Item class="StarterPack" referent="RBX56DD1F6CCCCC49738B0BAF8FEF9AD242">
		<Properties>
			<string name="Name">StarterPack</string>
		</Properties>
	</Item>
	<Item class="StarterGui" referent="RBX3D037C3B91F04FE781CF84D782569496">
		<Properties>
			<string name="Name">StarterGui</string>
			<bool name="ResetPlayerGuiOnSpawn">false</bool>
			<bool name="ShowDevelopmentGui">true</bool>
		</Properties>
	</Item>
	<Item class="TeleportService" referent="RBXA5FEB142F1B74552AD97A23726890E16">
		<Properties>
			<string name="Name">Teleport Service</string>
		</Properties>
	</Item>
	<Item class="SoundService" referent="RBXEFD02F9C23CD46A2B70151D3B53574B8">
		<Properties>
			<token name="AmbientReverb">0</token>
			<float name="DistanceFactor">10</float>
			<float name="DopplerScale">1</float>
			<string name="Name">SoundService</string>
			<float name="RolloffScale">1</float>
		</Properties>
	</Item>
	<Item class="CollectionService" referent="RBX3B7257264C414F2DB9882082046980C3">
		<Properties>
			<string name="Name">CollectionService</string>
		</Properties>
	</Item>
	<Item class="PhysicsService" referent="RBXA7DB1DDA31D24299864C13613C67AB57">
		<Properties>
			<string name="Name">PhysicsService</string>
		</Properties>
	</Item>
	<Item class="Geometry" referent="RBXE9D0D1EF1E0A4F0FBE64CCB2CDD3ADD2">
		<Properties>
			<string name="Name">Geometry</string>
		</Properties>
	</Item>
	<Item class="RenderHooksService" referent="RBX6419779B1A88431AA589E5A9C4457641">
		<Properties>
			<string name="Name">RenderHooksService</string>
		</Properties>
	</Item>
	<Item class="InsertService" referent="RBX1689FBCB3F264D9FA6A4B34DF7BD9029">
		<Properties>
			<bool name="AllowInsertFreeModels">false</bool>
			<string name="Name">InsertService</string>
		</Properties>
	</Item>
	<Item class="SocialService" referent="RBX137B197BC7F9479E9AEB6C992046A93F">
		<Properties>
			<string name="Name">SocialService</string>
		</Properties>
	</Item>
	<Item class="GamePassService" referent="RBX390E27BA2D724AE8BB6DCB5C679F2FD5">
		<Properties>
			<string name="Name">GamePassService</string>
		</Properties>
	</Item>
	<Item class="Debris" referent="RBX3B933DE2BF0D4E0D86A5A8B61F1EC871">
		<Properties>
			<int name="MaxItems">1000</int>
			<string name="Name">Debris</string>
		</Properties>
	</Item>
	<Item class="TimerService" referent="RBX570E0BBB75094E4EBA48D50D46697564">
		<Properties>
			<string name="Name">Instance</string>
		</Properties>
	</Item>
	<Item class="ScriptInformationProvider" referent="RBXF5E7FDEF386C4A24BC66B92AD66E0BFF">
		<Properties>
			<string name="Name">Instance</string>
		</Properties>
	</Item>
	<Item class="CookiesService" referent="RBXABF1BCDF9E464C0C9FF70431D9436148">
		<Properties>
			<string name="Name">CookiesService</string>
		</Properties>
	</Item>
	<Item class="ContextActionService" referent="RBX88E13CF273894B36838E6B0E827BED04">
		<Properties>
			<string name="Name">ContextActionService</string>
		</Properties>
	</Item>
	<Item class="ScriptService" referent="RBX968351041D4A4902BCF4AA3A25A94FF5">
		<Properties>
			<string name="Name">Instance</string>
		</Properties>
	</Item>
	<Item class="AssetService" referent="RBX373FDF69E96E465A81A06985B9646849">
		<Properties>
			<string name="Name">AssetService</string>
		</Properties>
	</Item>
	<Item class="Selection" referent="RBX3B193388605E4FFB8F3986D221361C88">
		<Properties>
			<string name="Name">Selection</string>
		</Properties>
	</Item>
	<Item class="ServerScriptService" referent="RBX84178D98AAC1473098EE017F66AC491B">
		<Properties>
			<bool name="LoadStringEnabled">false</bool>
			<string name="Name">ServerScriptService</string>
		</Properties>
	</Item>
	<Item class="ServerStorage" referent="RBX7CCFC70BCD434DC293C00D4564DB8868">
		<Properties>
			<string name="Name">ServerStorage</string>
		</Properties>
	</Item>
	<Item class="ReplicatedStorage" referent="RBXA196887506AD4A599B09EAA3FFAB0A88">
		<Properties>
			<string name="Name">ReplicatedStorage</string>
		</Properties>
	</Item>
	<Item class="LuaWebService" referent="RBXB1A5CD9463C54211951FAF559C8E8D76">
		<Properties>
			<string name="Name">Instance</string>
		</Properties>
	</Item>
	<Item class="Lighting" referent="RBXA81187996F3D4FAABAA731EBA8E2E166">
		<Properties>
			<Color3 name="Ambient">4278190080</Color3>
			<float name="Brightness">1</float>
			<Color3 name="ColorShift_Bottom">4278190080</Color3>
			<Color3 name="ColorShift_Top">4278190080</Color3>
			<Color3 name="FogColor">4290822336</Color3>
			<float name="FogEnd">100000</float>
			<float name="FogStart">0</float>
			<float name="GeographicLatitude">41.7332993</float>
			<bool name="GlobalShadows">true</bool>
			<string name="Name">Lighting</string>
			<Color3 name="OutdoorAmbient">4286611584</Color3>
			<bool name="Outlines">true</bool>
			<Color3 name="ShadowColor">4289967032</Color3>
			<string name="TimeOfDay">14:00:00</string>
		</Properties>
	</Item>
	<Item class="HttpService" referent="RBX422EACB3E8284D90BFA5D0AC166DC9A2">
		<Properties>
			<bool name="HttpEnabled">false</bool>
			<string name="Name">HttpService</string>
		</Properties>
	</Item>
	<Item class="GamepadService" referent="RBX013AD9D7D0694C9CB35A58F566A0409B">
		<Properties>
			<string name="Name">GamepadService</string>
		</Properties>
	</Item>
</roblox>
"#;
