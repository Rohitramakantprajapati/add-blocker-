module("luci.controller.voidblock", package.seeall)

function index()
	entry({"admin", "services", "voidblock"}, template("voidblock"), "VoidBlock", 90)
end
module("luci.controller.voidblock", package.seeall)

function index()
	entry({"admin", "services", "voidblock"}, template("voidblock"), "VoidBlock", 60).dependent = false
end
